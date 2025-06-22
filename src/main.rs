mod dir;
mod files;
mod printing;

use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};
use std::io::{self};
use std::path::{Path, PathBuf};

struct Cli {
    mode: String,
    path: std::path::PathBuf,
}

fn main() {}
