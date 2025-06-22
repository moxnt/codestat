#![allow(unused)]

use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};
use std::io::{self};
use std::path::{Path, PathBuf};

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
