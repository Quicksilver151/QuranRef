// extra utils
pub mod config;
pub mod data;
pub mod edit;
pub mod flag_parser;

pub use config::*;
pub use data::*;
pub use edit::*;
pub use flag_parser::*;

pub use crate::*;

// save data
use fs::File;
use io::Write;
use std::{fs, io};

pub fn save_to_data_dir(content: &str, filename: &str) -> std::io::Result<()> {
    // Get the project directories
    let project_dirs = ProjectDirs::from("", "", "quran-ref").unwrap();
    
    // Get the path to the data directory
    let data_dir = project_dirs.data_dir();
    
    // Create the full file path
    fs::create_dir_all(data_dir)?;
    let file_path = data_dir.join(filename);
    
    // Create the file and save the content
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    
    Ok(())
}

// terminal screen functions
pub fn clear_screen() {
    print!("\x1B[2J");
    print!("\x1b[1;1H");
}
pub fn new_buffer() {
    print!("\x1b[?1049h");
}
pub fn exit_buffer() {
    println!("\x1b[?1049l");
}

use signal_hook::{consts::SIGINT, iterator::Signals};
// handle SIGINT
pub fn handle_ctrlc() {
    let mut signals = Signals::new([SIGINT]).unwrap();
    
    std::thread::spawn(move || {
        for sig in signals.wait() {
            if sig == 2 {
                exit_buffer();
                std::process::exit(0)
            }
        }
    });
}

// input management
pub fn get_number_input() -> Result<u16, std::num::ParseIntError> {
    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    
    input_text.trim().parse::<u16>()
}
pub fn get_number_list_input() -> Result<Vec<u16>, std::num::ParseIntError> {
    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    
    input_text
        .split(',')
        .map(|x| x.trim().parse::<u16>())
        .collect()
}

pub fn remove_html_tags(input: &str) -> String {
    let mut result = String::new();
    let mut stack = Vec::new();
    let mut in_tag = false;
    
    for c in input.chars() {
        if c == '<' {
            in_tag = true;
            stack.push(c);
        } else if c == '>' {
            in_tag = false;
            stack.pop();
        } else if !in_tag {
            result.push(c);
        }
    }
    
    result
}

