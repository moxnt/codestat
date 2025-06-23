mod colors;

use colors::colored_string;
use std::collections::HashMap;
const LINE_LENGHT: f32 = 60.0;
const DEFAULT_COLOR: (u8, u8, u8) = (50, 200, 50);

pub fn print_stats(stats: HashMap<String, usize>, extensions: &HashMap<String, String>) {
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
                    format!("{} - {}", extension_to_language(key, extensions), value).as_str()
                )
                .as_str(),
                key,
                None
            ),
            colored_string("|".repeat(line_count).as_str(), key, Some(DEFAULT_COLOR)).as_str(),
            " ".repeat(LINE_LENGHT as usize - line_count),
            (proportion * 10000.0).round() / 100.0,
        );
    }
}

fn right_pad_space(width: usize, value: &str) -> String {
    format!("{}{}", value, " ".repeat(width - value.len()))
}

fn extension_to_language<'a>(string: &'a str, extensions: &'a HashMap<String, String>) -> &'a str {
    match extensions.get(string) {
        Some(language) => &language,
        None => string,
    }
}
