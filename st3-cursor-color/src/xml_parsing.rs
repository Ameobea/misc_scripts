//! Functions for parsing the XML-encoded SublimeText 3 theme file and modifying the cursor color from it.

use std::error::Error;
use std::fs::{copy, File};
use std::io::{self, BufReader, Read, Write};
use std::path::Path;

use xml::reader::{EventReader, XmlEvent};

fn get_old_caret_color_string(theme_path: &Path) -> Result<String, Box<Error>> {
    let file = File::open(theme_path)?;
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut found_caret = false;

    // find the current caret color string from the theme file
    for e in parser {
        match e {
            Ok(XmlEvent::Characters(chars)) => {
                if found_caret {
                    // this is the next string we've encountered after finding the string "caret" so this is the current caret color.
                    return Ok(chars)
                } else if chars == "caret" {
                    // finding this string means that the next string we find will be the hex color of the caret.
                    found_caret = true;
                }
            },
            Err(e) => { return Err(Box::new(e)) }
            _ => {}
        }
    }

    Err(Box::new(io::Error::new(io::ErrorKind::Other, "The string \"caret\" was not found in the file at the supplied path!")))
}

pub fn set_cursor_color(theme_path: &Path, new_color_hex: &str) -> Result<(), Box<Error>> {
    // find the value of the old caret color from the theme file
    let old_caret_color_string = get_old_caret_color_string(theme_path)?;

    // copy the old file to a backup before modifying
    let theme_file_name = theme_path
        .file_name()
        .expect("Supplied theme file path was not a file!")
        .to_str()
        .expect("Converting `OsStr` to `&str` returned `None`!");
    let mut backup_path_buf = theme_path.to_path_buf();
    backup_path_buf.set_file_name(&format!("{}_backup", theme_file_name));
    copy(theme_path, backup_path_buf)?;

    let mut file_content = String::new();
    let mut file = File::open(theme_path)?;
    // replace the old color string from the file with the new color string
    file.read_to_string(&mut file_content)?;
    let replaced_content = str::replace(&file_content, &old_caret_color_string, &new_color_hex);
    // Write the replaced content to the new file.
    let mut dst = File::create(theme_path)?;
    dst.write_all(replaced_content.as_str().as_bytes())?;

    Ok(())
}
