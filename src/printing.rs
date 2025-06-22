#![allow(unused)]

use rand;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

const LINE_LENGHT: f32 = 60.0;
const DEFAULT_COLOR: (u8, u8, u8) = (50, 200, 50);

fn print_stats(stats: &HashMap<String, usize>) {
    let total_lines: usize = stats.values().sum();
    let mut map_vec: Vec<_> = stats
        .iter()
        .filter(|x| *x.1 > (total_lines / 100))
        .collect();
    map_vec.sort_by(|a, b| a.1.cmp(b.1));

    println!("Total line count is {}", total_lines);
    for (key, value) in map_vec.iter().rev() {
        let proportion: f32 = **value as f32 / total_lines as f32;
        let line_count: usize = (proportion * LINE_LENGHT).round() as usize;
        println!(
            "{} [{}{}] - {}%",
            colored_string(
                right_pad_space(
                    20,
                    format!("{} - {}", extension_to_language(key), value).as_str()
                )
                .as_str(),
                key,
                None
            ),
            colored_string("|".repeat(line_count).as_str(), key, Some(DEFAULT_COLOR)).as_str(),
            " ".repeat(LINE_LENGHT as usize - line_count),
            (proportion * 10000.0).round() / 100.0,
        )
    }
}

fn get_extension_data() -> Option<HashMap<String, String>> {
    if let Ok(mut current_path) = env::current_exe() {
        current_path.pop();
        current_path.push("res/extensions.json");
        let path = current_path.as_path();
        return match read_to_string(path) {
            Ok(file) => Some(
                serde_json::from_str(&file)
                    .expect("extensions.json exists in installation, but is invalid"),
            ),
            Err(_) => None,
        };
    };

    None
}

fn right_pad_space(width: usize, value: &str) -> String {
    format!("{}{}", value, " ".repeat(width - value.len()))
}

fn extension_to_language(string: &str) -> String {
    // TODO Make this read extensions and their meanings from a file
    match get_extension_data() {
        Some(extensions) => match extensions.get(string) {
            Some(language) => language.to_string(),
            None => String::from(string),
        },
        None => {
            let value: &str = match string {
                "rs" => "Rust",
                "lua" => "Lua",
                "js" => "Javascript",
                "ts" => "Typescript",
                "c" => "C",
                "txt" => "Text",
                "html" => "HTML",
                "css" => "CSS3",
                unknown => unknown,
            };
            String::from(value)
        }
    }
}

pub fn colored_string(
    string: &str,
    color_identifier: &str,
    alternative: Option<(u8, u8, u8)>,
) -> String {
    let colors: HashMap<&str, Color> = HashMap::from([
        ("rs", Color::new(206, 66, 43)),
        ("py", Color::new(240, 219, 79)),
        ("ts", Color::new(0, 122, 204)),
        ("tsx", Color::new(0, 122, 204)),
        ("js", Color::new(240, 219, 79)),
        ("txt", Color::new(20, 80, 50)),
        ("html", Color::new(234, 90, 38)),
        ("css", Color::new(51, 169, 220)),
    ]);

    match colors.get(color_identifier) {
        Some(color) => format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            color.red, color.green, color.blue, string
        ),
        None => {
            let color: Color = match alternative {
                Some(alternative) => Color::new(alternative.0, alternative.1, alternative.2),
                None => Color::random(),
            };
            format!(
                "\x1b[38;2;{};{};{}m{}\x1b[0m",
                color.red, color.green, color.blue, string
            )
        }
    }
}

#[derive(Clone, Copy)]
struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    pub fn random() -> Self {
        let random_red: usize = rand::random();
        let random_green: usize = rand::random();
        let random_blue: usize = rand::random();

        Color {
            red: (random_red % 256) as u8,
            green: (random_green % 256) as u8,
            blue: (random_blue % 256) as u8,
        }
    }
}
