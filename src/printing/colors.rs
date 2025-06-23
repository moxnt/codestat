use rand;
use std::collections::HashMap;

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
