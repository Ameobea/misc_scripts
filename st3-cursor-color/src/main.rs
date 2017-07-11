//! Utility for automatically changing the color of the SublimeText 3 cursor based on the date.

extern crate chrono;
extern crate palette;
extern crate xml;

use std::env;
use std::path::Path;
use std::process::exit;

use chrono::{NaiveDate, Datelike, Utc};
use palette::{Gradient, Rgb};

pub mod xml_parsing;

fn date_to_frac(date: &NaiveDate) -> f32 { date.ordinal() as f32 / 365. }

fn hex_color(input: u8) -> String { if input < 16 { format!("0{:x}", input) } else { format!("{:x}", input) } }

fn rgb_to_hex(color: Rgb) -> String {
    format!("#{}{}{}", hex_color((color.red * 255.) as u8), hex_color((color.green * 255.) as u8), hex_color((color.blue * 255.) as u8))
}

fn create_gradient() -> Gradient<Rgb> {
    let mut gradient_steps = vec![
        (Rgb::new_u8(102, 109, 146), NaiveDate::from_ymd(2017, 01, 01)),
        (Rgb::new_u8(167, 233, 239), NaiveDate::from_ymd(2017, 02, 15)),
        (Rgb::new_u8(22, 175, 51), NaiveDate::from_ymd(2017, 06, 21)),
        (Rgb::new_u8(209, 92, 8), NaiveDate::from_ymd(2017, 10, 15)),
        (Rgb::new_u8(188, 11, 170), NaiveDate::from_ymd(2017, 04, 20)),
        (Rgb::new_u8(56, 70, 102), NaiveDate::from_ymd(2017, 11, 10)),
        (Rgb::new_u8(102, 109, 146), NaiveDate::from_ymd(2017, 12, 31))
    ];

    gradient_steps.sort_by(|&(_, date1), &(_, date2)| { date1.cmp(&date2) });

    let combined: Vec<(f32, Rgb)> = gradient_steps
        .into_iter()
        .map(|(color, date)| { (date_to_frac(&date), color,) })
        .collect();

    Gradient::with_domain(combined)
}

fn get_cur_hex() -> String {
    let cur_date = Utc::now();
    let cur_naive_date: NaiveDate = cur_date.naive_local().date();
    let gradient = create_gradient();
    rgb_to_hex(gradient.get(date_to_frac(&cur_naive_date)))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: `./st3-cursor-color /path/to/theme/file.tmTheme");
        exit(1)
    }

    let path = Path::new(&args[1]);
    // fetch the hex value of the current day
    let cur_hex = get_cur_hex();

    match xml_parsing::set_cursor_color(path, &cur_hex) {
        Ok(_) => (),
        Err(err) => {
            println!("Error while attempting to set cursor color in theme: {:?}", err);
            exit(1)
        },
    }
}

#[test]
fn print_weekly() {
    let gradient = create_gradient();
    let generated: Vec<(NaiveDate, Rgb)> = (0..(12 * 4)).map(|i| {
        let offset = i % 4;
        let m = ((i - offset) / 4) + 1;
        let d = (offset * 6) + 1;
        let date = NaiveDate::from_ymd(2017, m, d);
        (date, gradient.get(date_to_frac(&date)))
    }).collect();

    for (date, color) in generated {
        println!("{}: {}", date, rgb_to_hex(color));
    }
}
