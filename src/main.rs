
use std::env;
use std::path;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::Text;
use svg::node::element::path::Data;
use chrono::{Utc, Local, DateTime, Date};
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;


fn main() {
    // read config files
    let home = env::var("HOME").unwrap();
    let config_file_path  = format!("{}/.mydatestamp", home);
    if (std::path::Path::new(&config_file_path).exists() == false) {
        println!("prepare property file(~/.mydatestamp)");
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

    // check args
    let args: Vec<String> = env::args().collect();

    if (args.len() <= 1) {
        println!("usage: {} [your_name_and_affliation] [date] [output file]", args[0]);
        return;
    }

    let filename = &args[1];

    let center_circle = Data::new()
    .move_to((35, 10))
    .cubic_curve_by(((-25,0),(-25,25),(-25,25)))
    .cubic_curve_by(((0,25),(25,25),(25,25)))
    .cubic_curve_by(((25,0),(25,-25),(25,-25)))
    .cubic_curve_by(((0,-25),(-25,-25),(-25,-25))).close();

    let top_line = Data::new()
    .move_to((11,28))
    .line_by((48,00));

    let bottom_line = Data::new()
    .move_to((11,42))
    .line_by((48,00));

    let center_circle_path = Path::new()
    .set("fill", "none")
    .set("stroke", "red")
    .set("stroke-width", 1.2)
    .set("d", center_circle);

    let top_line_path = center_circle_path.clone()
    .set("d", top_line);

    let bottom_line_path = top_line_path.clone()
    .set("d", bottom_line);

    let date_str = Local::now().format("%Y/%m/%d").to_string();
    let text = svg::node::Text::new(date_str);
    let date_text = svg::node::element::Text::new()
    .set("font-family", "Lucida-Console")
    .set("font-size", 7.5)
    .set("fill", "red")
    .set("x",12)
    .set("y",37)
    .add(text);

    let top_text = svg::node::Text::new(top_line_str);
    let top_text_label = svg::node::element::Text::new()
    .set("font-family", "sans-serif")
    .set("font-size", 6)
    .set("fill", "red")
    .set("x",20)
    .set("y",20)
    .add(top_text);

    let top_text2 = svg::node::Text::new(top_line2_str);
    let top_text_label2 = svg::node::element::Text::new()
    .set("font-family", "sans-serif")
    .set("font-size", 5.5)
    .set("fill", "red")
    .set("x",14)
    .set("y",26)
    .add(top_text2);

    let bottom_text = svg::node::Text::new(username_str);
    let bottom_label = svg::node::element::Text::new()
    .set("font-family", "serif")
    .set("font-size", 8.5)
    .set("fill", "red")
    .set("x",18)
    .set("y",52)
    .add(bottom_text);

    let document = Document::new()
    .set("viewBox", (0, 0, 70, 70))
    .add(center_circle_path)
    .add(top_line_path)
    .add(bottom_line_path)
    .add(date_text)
    .add(top_text_label)
    .add(top_text_label2)
    .add(bottom_label);

    svg::save(filename.to_owned(), &document).unwrap();
    println!("wrote {} file", filename);
}
