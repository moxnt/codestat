mod string_colorizer;

use std::collections::HashMap;
use std::env::{self, args};
use std::fs::{read_dir, read_to_string};
use std::io::{self};
use std::path::{Path, PathBuf};

use string_colorizer::colored_string;
const LINE_LENGHT: f32 = 60.0;
const DEFAULT_COLOR: (u8, u8, u8) = (50, 200, 50);

fn main() {
    let mut stats: HashMap<String, usize> = HashMap::new();
    if let Some(path_string) = args().nth(1) {
        r_traverse_dir(Path::new(&path_string), &mut stats).unwrap();
    } else {
        r_traverse_dir(Path::new("./src"), &mut stats).unwrap();
    }
    print_stats(&stats);
}

fn r_traverse_dir(path: &Path, stats: &mut HashMap<String, usize>) -> io::Result<()> {
    let entries = read_dir(path)?;
    //let gitignore_path = path.to_path_buf().push(".gitignore/");
    for entry in entries {
        let path = entry?.path();
        if path.is_dir() {
            r_traverse_dir(path.as_path(), stats)?;
        } else {
            handle_file(path, stats);
        }
    }
    Ok(())
}

fn handle_file(file_path: PathBuf, stats: &mut HashMap<String, usize>) {
    let (_, extension) = file_path
        .to_str()
        .unwrap()
        .rsplit_once('.')
        .expect("No file extension");

    if extension.contains('/') {
        // This is in case a file does not have an extension
        return;
    }

    let file = read_to_string(&file_path);
    if let Err(_) = file {
        return;
    }

    let line_count: usize = file.unwrap().lines().count();

    if line_count > 20 {
        if let Some(field) = stats.get_mut(extension) {
            *field = *field + line_count;
        } else {
            stats.insert(extension.to_string(), line_count);
        }
    }
}

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
    //
    //
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
