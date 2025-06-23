#![allow(unused)]

use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};
use std::io::{self};
use std::path::{Path, PathBuf};
use std::usize;

pub fn process_files<T: Iterator<Item = String>>(files: T) -> HashMap<String, usize> {
    let mut stats: HashMap<String, usize> = HashMap::new();
    for file in files {
        handle_file(PathBuf::from(file), &mut stats);
    }
    stats
}

pub fn process_directory(path: PathBuf) -> io::Result<HashMap<String, usize>> {
    let mut stats: HashMap<String, usize> = HashMap::new();
    let entries = read_dir(path)?;
    for entry in entries {
        let path = entry?.path();
        if path.is_dir() {
            r_traverse_dir(path, &mut stats);
        } else {
            handle_file(path, &mut stats);
        }
    }
    Ok(stats)
}

pub fn r_traverse_dir(path: PathBuf, stats: &mut HashMap<String, usize>) -> io::Result<()> {
    let entries = read_dir(path)?;
    for entry in entries {
        let path = entry?.path();
        if path.is_dir() {
            r_traverse_dir(path, stats)?;
        } else {
            handle_file(path, stats);
        }
    }
    Ok(())
}

fn handle_file(file_path: PathBuf, stats: &mut HashMap<String, usize>) {
    if let Some((filename, extension)) = file_path.to_str().unwrap().rsplit_once('.') {
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
                *field += line_count;
            } else {
                stats.insert(extension.to_string(), line_count);
            }
        }
    } else {
        println!("{:?} has no extension", file_path);
    }
}
