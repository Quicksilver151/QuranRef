// std
use std::env;

// crates
pub use colored::*;
pub use directories::*;
pub use serde::{Deserialize, Serialize};

// include files
mod api;
mod models;
mod utils;

// use files
use api::*;
use models::*;
use utils::*;


#[allow(unused)]
fn main() {
    // load config & data
    let downloaded_tls = get_downloaded_translations_list();
    let mut cfg = Config::load();
    
    // fetch flags
    let args: Vec<String> = env::args().collect();
    let flag: Flag = match flag_parser::parse_args(args) {
        Ok(flag) => flag,
        Err(_flag) => {
            println!("flag_err:{_flag:?}\n{}", HELP_TEXT);
            return;
        }
    };
    
    // branch flags
    if flag.help {
        println!("{}", HELP_TEXT);
        return;
    }
    if flag.edit {
        edit(&mut cfg);
        return;
    }
    if flag.download {
        download_translation();
        println!("Download complete");
        return;
    }
    if flag.arabic {
        todo!("display verses in arabic");
    }
    if flag.verses.index.chapter == 0 {
        println!("{}", HELP_TEXT);
        return;
    }
    if flag.verses.is_in_order() {
        println!("fetching verses:");
    } else {
        todo!("handle verses in reverse order");
    }
    
    if cfg.selected_tls.is_empty() {
        eprintln!(
            "{}",
            "Download some translations with -d and select them with -e".red()
        )
    }
    let quran_tls: Vec<Quran> = cfg
        .selected_tls
        .iter()
        .map(load_downloaded_translation)
        .collect();
    
    show_verses(quran_tls, &flag.verses);
}

pub fn show_verses(quran: Vec<Quran>, verse_range: &VerseRange) {
    
    // do not touch dis mess!!!!!!!!!!!1
    let tls: Vec<Vec<Verse>> = quran.iter().map(|q| q.get_slice(verse_range)).collect();
    let verse_num = tls[0].len();
    for verse in 0..verse_num {
        let index = format!("|{}|", tls[0][verse].index).bold().on_black();
        let index_len = index.chars().count();
        let equal_len = 64 - index_len;
        let eq_string = "=".repeat(equal_len);
        
        let text = format!("{}{}", index, eq_string.red());
        println!("{text}");
        for tl in tls.iter() {
            let out = &tl[verse];
            println!("{}", out);
            
            if tl != tls.last().unwrap() {
                println!("{}", "-".repeat(64).black());
            }
        }
    }
    println!("{}", "=".repeat(64).red());
}

