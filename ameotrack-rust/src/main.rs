use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

use arboard::Clipboard;
use chrono::{DateTime, TimeDelta, Utc};
use clap::{Parser, Subcommand};
use colored::Colorize;
use reqwest::multipart;
use serde::Deserialize;
use serde_json::Value;
use tokio;

#[derive(Deserialize)]
struct Config {
    ameotrack_url_root: String,
    upload_password: String,
}

async fn make_request(
    method: &str,
    url: &str,
    params: Option<&HashMap<&str, &str>>,
    json_body: Option<Value>,
    form_data: Option<multipart::Form>,
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let request_builder = match method {
        "POST" => client.post(url),
        "GET" => client.get(url),
        "PUT" => client.put(url),
        "PATCH" => client.patch(url),
        "DELETE" => client.delete(url),
        _ => unimplemented!("Invalid HTTP method"),
    };

    let request_builder = if let Some(params) = params {
        request_builder.query(params)
    } else {
        request_builder
    };

    let request_builder = if let Some(json_body) = json_body {
        request_builder.json(&json_body)
    } else {
        request_builder
    };

    let request_builder = if let Some(form_data) = form_data {
        request_builder.multipart(form_data)
    } else {
        request_builder
    };

    let response = request_builder.send().await?;
    let text = response.text().await?;
    Ok(text)
}

async fn api_call(
    conf: &Config,
    resource_path: &str,
    method: &str,
    params: Option<&HashMap<&str, &str>>,
    json_body: Option<Value>,
    form_data: Option<multipart::Form>,
) -> Result<String, reqwest::Error> {
    let url = format!("{}/{}", conf.ameotrack_url_root, resource_path);
    make_request(method, &url, params, json_body, form_data).await
}

fn load_conf() -> Config {
    let conf_file_path = "~/.ameotrack/conf.toml";
    let conf_file_path = shellexpand::tilde(conf_file_path).to_string();
    let mut file = File::open(conf_file_path).expect("Could not open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read config file");

    toml::from_str(&contents).expect("Error reading config file")
}

fn copy_to_clipboard(text: &str) {
    if cfg!(target_os = "linux") {
        // if `xclip` is installed, use it to copy to clipboard
        let _ = std::process::Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .unwrap()
            .stdin
            .unwrap()
            .write_all(text.as_bytes());
    } else {
        let mut clipboard = Clipboard::new().unwrap();
        let _ = clipboard.set_text(text).unwrap();
    }
}

async fn upload_command(
    conf: &Config,
    filename: &Path,
    one_time: bool,
    private: bool,
    expiry: Option<i32>,
) {
    let expiry: i32 = expiry.unwrap_or(-1);

    let file = tokio::fs::read(filename)
        .await
        .expect("Could not read file");
    let filename = filename.file_name().unwrap();
    let filename = filename.to_str().unwrap().to_owned();
    let file_part = multipart::Part::bytes(file).file_name(filename);
    let form_data = multipart::Form::new()
        .part("file", file_part)
        .text("expiry", expiry.to_string())
        .text("secret", if private { "1" } else { "" }.to_string())
        .text("oneTime", if one_time { "1" } else { "" }.to_string())
        .text("password", conf.upload_password.clone())
        .text("source", "at-cli".to_string());

    match api_call(&conf, "upload", "POST", None, None, Some(form_data)).await {
        Ok(url) => {
            copy_to_clipboard(&url);
            println!("{} {url}", "File successfully uploaded:".green());
        }
        Err(err) => {
            eprintln!("{} {err}", "Error uploading file:".red());
        }
    }
}

async fn remind_command(conf: &Config, include_timestamp: bool, date: &str, message: &str) {
    let mut message = message.to_string();
    if include_timestamp {
        let time_str = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        message.push_str(&format!("\nSent at {}", time_str));
    }

    let params = [("dateString", date), ("message", &message)]
        .iter()
        .cloned()
        .collect();
    match api_call(&conf, "remind", "GET", Some(&params), None, None).await {
        Ok(res) => match serde_json::from_str::<Value>(&res) {
            Ok(json) => {
                if json["success"].as_bool().unwrap_or(false) {
                    let timestamp = json["timestamp"].as_i64().unwrap();
                    let delivery_dt =
                        DateTime::<Utc>::from_timestamp(timestamp, 0).expect("Invalid timestamp");
                    let mut tdelta = delivery_dt - Utc::now();
                    tdelta = TimeDelta::seconds(tdelta.num_seconds() + 1);
                    println!(
                        "{} {}",
                        "Reminder successfully created; will be sent in:".green(),
                        humantime::format_duration(tdelta.to_std().unwrap())
                    );
                } else {
                    eprint!(
                        "Error creating reminder: {}",
                        json["reason"].as_str().unwrap_or("Unknown error")
                    );
                }
            }
            Err(_) => {
                eprintln!("Received bad response from server: {res}");
            }
        },
        Err(err) => {
            eprintln!("Error creating reminder: {err}");
        }
    }
}

lazy_static::lazy_static! {
    static ref BIN_NAME_RGX: regex::Regex = regex::Regex::new(r#"; url=\.(.*)""#).unwrap();
}

async fn bin_command(conf: &Config, password: &str, file_path: &Path, secret: bool) {
    let filename = file_path.file_name().unwrap().to_owned();
    let filename = filename.to_str().unwrap().to_owned();
    let text = tokio::fs::read_to_string(file_path)
        .await
        .expect("Could not read file");

    let mut json_data = serde_json::Map::new();
    json_data.insert("source".to_string(), "at-cli".into());
    json_data.insert("filename".to_string(), filename.into());
    json_data.insert("password".to_string(), password.into());
    json_data.insert("text".to_string(), text.into());
    if secret {
        json_data.insert("secret".to_string(), "1".into());
    }

    match api_call(
        &conf,
        "bin",
        "POST",
        None,
        Some(serde_json::Value::Object(json_data)),
        None,
    )
    .await
    {
        Ok(res) => {
            let url_match = BIN_NAME_RGX
                .captures(&res)
                .expect("Could not extract bin URL")
                .get(1)
                .expect("Could not extract bin URL")
                .as_str();
            let bin_url = format!("{}{url_match}", conf.ameotrack_url_root);
            let mut clipboard = Clipboard::new().unwrap();
            let _ = clipboard.set_text(&bin_url);
            println!("{} {bin_url}", "Bin successfully created:".green());
        }
        Err(err) => {
            eprintln!("{} {err}", "Error creating bin:".red());
            return;
        }
    }
}

#[derive(Parser)]
#[command(name = "upload")]
struct UploadCommand {
    #[arg(short, long)]
    one_time: bool,
    #[arg(short, long)]
    private: bool,
    #[arg(short, long)]
    expiry: Option<i32>,
    filename: PathBuf,
}

#[derive(Parser)]
#[command(name = "remind")]
struct RemindCommand {
    #[arg(short, long)]
    timestamp: bool,
    date: String,
    message: String,
}

#[derive(Parser)]
#[command(name = "bin")]
struct BinCommand {
    #[arg(short, long)]
    secret: bool,
    password: String,
    file_path: PathBuf,
}

#[derive(Subcommand)]
enum Command {
    Upload(UploadCommand),
    Remind(RemindCommand),
    Bin(BinCommand),
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Cli::parse();

    let conf = load_conf();

    match args.command {
        Command::Upload(upload) => {
            upload_command(
                &conf,
                &upload.filename,
                upload.one_time,
                upload.private,
                upload.expiry,
            )
            .await;
        }
        Command::Remind(remind) => {
            remind_command(&conf, remind.timestamp, &remind.date, &remind.message).await;
        }
        Command::Bin(bin) => {
            bin_command(&conf, &bin.password, &bin.file_path, bin.secret).await;
        }
    }

    // match matches.subcommand() {
    //     ("upload", Some(sub_m)) => upload_command(sub_m, &state).await,
    //     ("remind", Some(sub_m)) => remind_command(sub_m, &state).await,
    //     ("bin", Some(sub_m)) => bin_command(sub_m, &state).await,
    //     _ => println!("{}", matches.usage()),
    // }
}
