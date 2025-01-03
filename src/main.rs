mod string_colorizer;

use std::collections::HashMap;
use std::env::args;
use std::fs::{read_dir, read_to_string};
use std::io;
use std::path::{Path, PathBuf};

use string_colorizer::colored_string;
const LINE_LENGHT: f32 = 60.0;

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

    let file = read_to_string(&file_path);
    if let Err(_) = file {
        return;
    }

    let line_count: usize = file.unwrap().lines().count();

    if let Some(field) = stats.get_mut(extension) {
        *field = *field + line_count;
    } else {
        stats.insert(extension.to_string(), line_count);
    }
}

fn print_stats(stats: &HashMap<String, usize>) {
    let total_lines: usize = stats.values().sum();
    println!("Total line count is {}", total_lines);
    for (key, value) in stats.iter() {
        let proportion: f32 = *value as f32 / total_lines as f32;
        let line_count: usize = (proportion * LINE_LENGHT).round() as usize;
        println!(
            "{} [{}{}] - {}%",
            colored_string(
                right_pad_space(
                    20,
                    format!("{} - {}", extension_to_language(key), value).as_str()
                )
                .as_str(),
                key
            ),
            colored_string("|".repeat(line_count).as_str(), key).as_str(),
            " ".repeat(LINE_LENGHT as usize - line_count),
            (proportion * 10000.0).round() / 100.0,
        )
    }
}

fn right_pad_space(width: usize, value: &str) -> String {
    format!("{}{}", value, " ".repeat(width - value.len()))
}

fn extension_to_language(string: &str) -> String {
    // TODO Make this read extensions and their meanings from a file
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
