
use std::env;
use std::io::stderr;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;
use chrono::Local;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    // check args
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        let usage = format!("[Err] Args not found.\nusage: {} [output file]\n", args[0]);
        stderr().write(&usage.as_bytes());
        return;
    }

    // read config files
    let home = env::var("HOME").unwrap();
    let config_file_path = format!("{}/.mydatestamp", home);
    if std::path::Path::new(&config_file_path).exists() == false {
        stderr().write(b"[Err] Property file not found. Please prepare your property file(~/.mydatestamp)\n");
        return;
    }
    println!("reading property file(~/.mydatestamp)");

    let f = File::open(&config_file_path).unwrap();
    let mut br = BufReader::new(f);

    let mut top_line_str = String::new();
    br.read_line(&mut top_line_str);
    let mut top_line2_str = String::new();
    br.read_line(&mut top_line2_str);
    let mut username_str = String::new();
    br.read_line(&mut username_str);

    print!("top line 1:{}", top_line_str);
    print!("top line 2:{}", top_line2_str);
    print!("username:{}", username_str);
    let filename = &args[1];

    // build elements of SVG nodes
    let center_circle = Data::new()
        .move_to((105, 30))
        .cubic_curve_by(((-75, 0), (-75, 75), (-75, 75)))
        .cubic_curve_by(((0, 75), (75, 75), (75, 75)))
        .cubic_curve_by(((75, 0), (75, -75), (75, -75)))
        .cubic_curve_by(((0, -75), (-75, -75), (-75, -75)))
        .close();

    let top_line = Data::new().move_to((11 * 3, 28 * 3)).line_by((48 * 3, 0));

    let bottom_line = Data::new().move_to((11 * 3, 42 * 3)).line_by((48 * 3, 00));

    let center_circle_path = Path::new()
        .set("fill", "none")
        .set("stroke", "red")
        .set("stroke-width", 3.6)
        .set("d", center_circle);

    let top_line_path = center_circle_path.clone().set("d", top_line);

    let bottom_line_path = top_line_path.clone().set("d", bottom_line);

    let text_base = svg::node::element::Text::new()
        .set("font-family", "sans-serif")
        .set("font-size", 22)
        .set("fill", "red");

    let date_str = Local::now().format("%Y/%m/%d").to_string();
    let date_text = svg::node::Text::new(date_str);
    let date_label = text_base.clone().set("x", 12 * 3).set("y", 37 * 3).add(
        date_text,
    );

    let top_text = svg::node::Text::new(top_line_str);
    let top_text_label = text_base
        .clone()
        .set("x", 63)
        .set("y", 60)
        .set("font-size", 20)
        .add(top_text);

    let top_text2 = svg::node::Text::new(top_line2_str);
    let top_text_label2 = text_base
        .clone()
        .set("x", 46)
        .set("y", 78)
        .set("font-size", 18)
        .add(top_text2);

    let bottom_text = svg::node::Text::new(username_str);
    let bottom_label = text_base
        .clone()
        .set("font-family", "serif")
        .set("x", 18 * 3)
        .set("y", 52 * 3)
        .add(bottom_text);


    // add elements to document
    let document = Document::new()
        .set("viewBox", (0, 0, 210, 210))
        .add(center_circle_path)
        .add(top_line_path)
        .add(bottom_line_path)
        .add(date_label)
        .add(top_text_label)
        .add(top_text_label2)
        .add(bottom_label);

    // write file
    svg::save(filename.to_owned(), &document).unwrap();
    println!("wrote {} file", filename);

}
