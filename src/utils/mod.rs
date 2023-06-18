// extra utils
pub mod flag_parser;
pub mod config;
pub mod data;

pub use flag_parser::*;
pub use config::*;
pub use data::*;


// utils

// save data
use directories::ProjectDirs;
use std::{fs,io};
use fs::File;
use io::Write;

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
#[allow(dead_code)]
pub fn new_buffer() {
    print!("\x1b[?1049h");
}
#[allow(dead_code)]
pub fn exit_buffer() {
    print!("\x1b[?1049l");
}


use signal_hook::{consts::SIGINT, iterator::Signals};
// handle SIGINT
#[allow(dead_code)]
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
