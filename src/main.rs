extern crate clap;

use clap::{App, Arg};
use std::str::FromStr;

mod img;
use img::Color;

fn main() {
    if let Err(n) = command() {
        println!("{}", n);
    }
}

fn command() -> Result<(), String> {
    let matches = App::new("Sample image generator")
        .version("1.0.0")
        .author("LeafChage (https://github.com/LeafChage)")
        .arg(
            Arg::with_name("output")
            .short("o")
            .long("output")
            .help("write image path")
            .value_name("FILE path")
            .required(true),
        )
        .arg(
            Arg::with_name("width")
            .short("w")
            .long("width")
            .value_name("number")
            .help("sample image width")
            .required(true),
        )
        .arg(
            Arg::with_name("height")
            .short("h")
            .long("height")
            .value_name("number")
            .help("sample image height")
            .required(true),
        )
        .arg(
            Arg::with_name("color")
            .short("c")
            .long("color")
            .help("default is gray")
            .value_name("rgb(uint, uint, uint) | black| blue| cyan| gray| green| magenta| red| white| yellow")
            .required(false),
        )
        .get_matches();

    let path = if let Some(path) = matches.value_of("output") {
        path
    } else {
        return Err(String::from("required path"));
    };

    let color = match matches.value_of("color") {
        Some("black") => Color::Black,
        Some("blue") => Color::Blue,
        Some("cyan") => Color::Cyan,
        Some("gray") => Color::Gray,
        Some("green") => Color::Green,
        Some("magenta") => Color::Magenta,
        Some("red") => Color::Red,
        Some("white") => Color::White,
        Some("yellow") => Color::Yellow,
        Some(_) => Color::Default,
        // Other(u8, u8, u8),
        _ => Color::Default,
    };

    let width = if let Some(width) = matches.value_of("width") {
        if let Ok(n) = u32::from_str(width) {
            n
        } else {
            return Err(String::from("width is u32"));
        }
    } else {
        return Err(String::from("required width"));
    };

    let height = if let Some(height) = matches.value_of("height") {
        if let Ok(n) = u32::from_str(height) {
            n
        } else {
            return Err(String::from("width is u32"));
        }
    } else {
        return Err(String::from("required height"));
    };

    let result = img::generate(
        path,
        img::color(color),
        width,
        height,
        &format!("{}x{}", width, height),
    );
    println!("{:?}", matches.value_of("output"));
    println!("{:?}", matches.value_of("color"));
    println!("{:?}", matches.value_of("width"));
    println!("{:?}", matches.value_of("height"));
    println!("{}", path);

    return match result {
        Err(n) => Err(format!("{}", n)),
        _ok => Ok(()),
    };
}
