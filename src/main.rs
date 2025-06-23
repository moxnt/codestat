mod counters;
mod printing;

use crate::counters::*;
use clap::{CommandFactory, Parser};
use std::collections::HashMap;
use std::io::{stdin, BufRead, BufReader, IsTerminal};
use std::path::PathBuf;

use crate::printing::print_stats;

#[derive(Parser, Debug)]
#[command(about = "A code analysis tool", long_about = None)]
#[command(version = "0.2")]
struct Cli {
    /// Path of the file / dir use - instead of an actual path to accept a piped list of files.
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let extensions = include_str!("../res/extensions.json");
    let extension_data: HashMap<String, String> =
        serde_json::from_str(extensions).expect("serde_json failed");
    let stats = if args.path == PathBuf::from("-") {
        if stdin().is_terminal() {
            Cli::command().print_help().unwrap();
            ::std::process::exit(2);
        }

        let files = BufReader::new(stdin().lock())
            .lines()
            .map(|l| l.expect("Problem with stdin lines"));
        process_files(files)
    } else {
        process_directory(args.path).expect("Got an IO error")
    };

    print_stats(stats, &extension_data);
}
