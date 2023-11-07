use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use chrono::Local;
use svg::{Document, node::element::Path, node::element::path::Data, node::element::Text};

const CONFIG_FILE: &str = ".mydatestamp";
const SVG_STROKE_RED: &str = "red";
const SVG_FONT_FAMILY_SANS: &str = "sans-serif";
const SVG_FONT_FAMILY_SERIF: &str = "serif";
const SVG_FONT_SIZE_LARGE: i32 = 22;
const SVG_FONT_SIZE_MEDIUM: i32 = 20;
const SVG_FONT_SIZE_SMALL: i32 = 18;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let output_file = match args.get(1) {
        Some(filename) => filename,
        None => return Err(From::from("Usage: mydatestamp [output file]")),
    };

    let config_file_path = get_config_file_path()?;
    let lines = read_config(&config_file_path)?;

    let document = build_svg(&lines)?;
    svg::save(output_file, &document)?;

    Ok(())
}

fn get_config_file_path() -> Result<String, io::Error> {
    let home = match env::var("HOME") {
        Ok(val) => val,
        Err(_) => return Err(io::Error::new(io::ErrorKind::NotFound, "HOME environment variable not set")),
    };

    let config_file_path = format!("{}/{}", home, CONFIG_FILE);
    if !std::path::Path::new(&config_file_path).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Property file not found. Please prepare your property file(~/.mydatestamp)"));
    }

    Ok(config_file_path)
}

fn read_config(config_file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(config_file_path)?;
    let mut reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for _ in 0..3 {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break; // EOF reached or no more lines to read
        }
        lines.push(line.trim_end().to_string()); // Trim the newline character and push to the vector
    }

    if lines.len() < 3 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Config file has fewer than 3 lines.",
        ));
    }

    Ok(lines)
}

fn build_svg(lines: &[String]) -> Result<Document, Box<dyn std::error::Error>> {
    if lines.len() < 3 {
        return Err("Not enough lines in the config file".into());
    }

    let center_circle_path = build_center_circle_path();
    let top_line_path = build_top_line_path();
    let bottom_line_path = build_bottom_line_path();

    let top_text_label = build_text_element(&lines[0], 63, 60, SVG_FONT_FAMILY_SANS, SVG_FONT_SIZE_MEDIUM, SVG_STROKE_RED);
    let top_text_label2 = build_text_element(&lines[1], 46, 78, SVG_FONT_FAMILY_SANS, SVG_FONT_SIZE_SMALL, SVG_STROKE_RED);
    let bottom_text_label = build_text_element(&lines[2], 18 * 3, 52 * 3, SVG_FONT_FAMILY_SERIF, SVG_FONT_SIZE_SMALL, SVG_STROKE_RED);

    let date_str = Local::now().format("%Y/%m/%d").to_string();
    let date_label = build_text_element(&date_str, 12 * 3, 37 * 3, SVG_FONT_FAMILY_SANS, SVG_FONT_SIZE_LARGE, SVG_STROKE_RED);

    let document = Document::new()
        .set("viewBox", (0, 0, 210, 210))
        .add(center_circle_path)
        .add(top_line_path)
        .add(bottom_line_path)
        .add(date_label)
        .add(top_text_label)
        .add(top_text_label2)
        .add(bottom_text_label);

    Ok(document)
}

fn build_center_circle_path() -> Path {
    let center_circle = Data::new()
        .move_to((105, 30))
        .cubic_curve_by(((-75, 0), (-75, 75), (-75, 75)))
        .cubic_curve_by(((0, 75), (75, 75), (75, 75)))
        .cubic_curve_by(((75, 0), (75, -75), (75, -75)))
        .cubic_curve_by(((0, -75), (-75, -75), (-75, -75)))
        .close();

    return Path::new()
        .set("fill", "none")
        .set("stroke", "red")
        .set("stroke-width", 3.6)
        .set("d", center_circle);
}

fn build_top_line_path() -> Path {
    let top_line = Data::new().move_to((11 * 3, 28 * 3)).line_by((48 * 3, 0));
    return Path::new().set("d", top_line)
        .set("fill", "none")
        .set("stroke", "red")
        .set("stroke-width", 3.6);
}

fn build_bottom_line_path() -> Path {
    let bottom_line = Data::new().move_to((11 * 3, 42 * 3)).line_by((48 * 3, 00));
    return Path::new().set("d", bottom_line)
        .set("fill", "none")
        .set("stroke", "red")
        .set("stroke-width", 3.6);
}


fn build_text_element(data: &str, x: i32, y: i32, font_family: &str, font_size: i32, fill: &str) -> Text {
    return Text::new()
        .set("x", x)
        .set("y", y)
        .set("font-family", font_family)
        .set("font-size", font_size)
        .set("fill", fill)
        .add(svg::node::Text::new(data))
}