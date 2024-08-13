use colorize::AnsiColor;
use inquire;
use std::{fmt::Display, io};
use walkdir::WalkDir;

fn directory_search(search_term_name: &str) -> io::Result<()> {
    for entry in WalkDir::new("C:\\Users").into_iter().filter_map(|e| e.ok()) {
        println!("Searching");
        let path = entry.path();
        if path.file_name() == Some(search_term_name.as_ref()) {
            if path.is_file() {
                format!("Found Expected File {}", path.display());
                return Ok(());
            }
            if path.is_dir() {
                format!("Found Expected Directory {}", path.display());
                return Ok(());
            }
        }
    }
    Ok(())
}

fn main() {
    let prompt_file_term = "What file name are you searching for".yellow();

    let prompt = inquire::prompt_text(prompt_file_term);
    directory_search(prompt.expect("Failure").as_str());
}
