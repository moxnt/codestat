use std::collections::HashMap;

pub fn colored_string(string: &str, color_identifier: &str) -> String {
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
        None => String::from(string),
    }
}

struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }
}
