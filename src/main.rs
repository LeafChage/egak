extern crate clap;

use clap::{App, Arg, ArgMatches};
use std::str::FromStr;

mod img;
use img::Color;

fn main() {
    if let Err(n) = command() {
        println!("{}", n);
    }
}

const RESOLUTION_4K: (u32, u32) = (4096, 2160);

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
                .value_name(
                    "#ffffff | black | blue | cyan | gray | green | magenta | red | white | yellow",
                )
                .required(false),
        )
        .arg(
            Arg::with_name("force")
                .short("f")
                .long("force")
                .help("allow to generate over 4k resolution (4096x2160)")
                .value_name("1 | 0")
                .required(false),
        )
        .get_matches();
    let path = fetch_string(&matches, "output")?;
    let color = fetch_color(&matches)?;
    let width = fetch_u32(&matches, "width")?;
    let height = fetch_u32(&matches, "height")?;
    let force = if let Ok(f) = fetch_u32(&matches, "force") {
        f == 1
    } else {
        false
    };

    if !force {
        let (w_4k, h_4k) = RESOLUTION_4K;
        if !(width < w_4k && height < h_4k) {
            return Err(String::from(
                "must not to generate over 4k resolution, you need to specify force flag = 1.",
            ));
        }
    }

    let result = img::generate(
        path,
        color.rgb(),
        width,
        height,
        &format!("{}x{}", width, height),
    );
    println!("{}", path);

    return match result {
        Err(n) => Err(format!("{}", n)),
        _ok => Ok(()),
    };
}

#[test]
fn test() {
    img::generate(
        "./test0.png",
        Color::Default.rgb(),
        10,
        10,
        &format!("{}x{}", 10, 10),
    );
    img::generate(
        "./test1.png",
        Color::Default.rgb(),
        100,
        200,
        &format!("{}x{}", 100, 200),
    );
    img::generate(
        "./test2.png",
        Color::Default.rgb(),
        200,
        100,
        &format!("{}x{}", 200, 100),
    );
    img::generate(
        "./test3.png",
        Color::Default.rgb(),
        1200,
        200,
        &format!("{}x{}", 1200, 200),
    );
    img::generate(
        "./test4.png",
        Color::Default.rgb(),
        200,
        1200,
        &format!("{}x{}", 200, 1200),
    );
}

fn fetch_string<'a>(matches: &'a ArgMatches, name: &str) -> Result<&'a str, String> {
    if let Some(path) = matches.value_of(name) {
        Ok(path)
    } else {
        Err(format!("required {}", name))
    }
}

fn fetch_u32(matches: &ArgMatches, name: &str) -> Result<u32, String> {
    if let Some(value) = matches.value_of(name) {
        if let Ok(n) = u32::from_str(value) {
            Ok(n)
        } else {
            Err(format!("{} is not u32", name))
        }
    } else {
        Err(format!("required {}", name))
    }
}

fn fetch_color(matches: &ArgMatches) -> Result<Color, String> {
    match matches.value_of("color") {
        Some("black") => Ok(Color::Black),
        Some("blue") => Ok(Color::Blue),
        Some("cyan") => Ok(Color::Cyan),
        Some("gray") => Ok(Color::Gray),
        Some("green") => Ok(Color::Green),
        Some("magenta") => Ok(Color::Magenta),
        Some("red") => Ok(Color::Red),
        Some("white") => Ok(Color::White),
        Some("yellow") => Ok(Color::Yellow),
        Some(n) => {
            let src = n.as_bytes();
            if src[0] != b'#' || src.len() != 7 {
                return Err(String::from("color format is #ffffff | black | blue | cyan | gray | green | magenta | red | white | yellow"));
            }

            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str(&format!("{}{}", src[1], src[2])),
                u8::from_str(&format!("{}{}", src[3], src[4])),
                u8::from_str(&format!("{}{}", src[5], src[6])),
            ) {
                Ok(Color::Other(r, g, b))
            } else {
                Err(String::from("color format is #ffffff"))
            }
        }
        _ => Ok(Color::Default),
    }
}
