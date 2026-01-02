// std
use std::env;

use clap::Parser;
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

    let arg = cli::Cli::parse();



    // let cli = cli::Cli::parse();
    //
    // let verses = cli.verse_range()?;
    //
    //
    // // load config & data
    // let downloaded_tls = get_downloaded_translations_list();
    let mut cfg = Config::load();
    //
    //
    // // fetch flags
    // let args: Vec<String> = env::args().collect();
    // let flag: Flag = cli::parse_args(args).unwrap();

    if arg.edit {
        edit(&mut cfg);
        return;
    }
    if arg.download {
        download_translation();
        println!("Download complete");
        return;
    }
    // if flag.arabic {
    //     todo!("display verses in arabic");
    // }
    // if flag.verses.index.chapter == 0 {
    //     println!("{}", HELP_TEXT);
    //     return;
    // }
    // if flag.verses.is_in_order() {
    //     println!("fetching verses:");
    // } else {
    //     todo!("handle verses in reverse order");
    // }
    
    /// Config empty
    if cfg.selected_tls.is_empty() {
        eprintln!(
            "{}",
            "Download some translations with -d and select them with -e".red()
        )
    }

    let verses = match arg.verse_range(){
        Ok(verse) => verse,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    let quran_tls: Vec<Quran> = cfg
        .selected_tls
        .iter()
        .map(load_downloaded_translation)
        .collect();

    if arg.raw {
        show_verses_raw(quran_tls, &verses);
    } else {
        show_verses(quran_tls, &verses);
    }

    // Ok(())
}

pub fn show_verses_raw(quran: Vec<Quran>, verse_range: &VerseRange) {

    let tls: Vec<Vec<Verse>> = quran.iter().map(|q| q.get_slice(verse_range)).collect();
    let verse_num = tls[0].len();
    for tl in tls.iter() {
        println!("\n{}", tl[0].tl.to_string().red());
        for verse in 0..verse_num {
            let index = format!("{}", tls[0][verse].index.to_string().bold().green());
            print!("{index} ");
            let out: &Verse = &tl[verse];
            println!("{}", out);


            // if tl != tls.last().unwrap() {
            //     println!("{}", "-".repeat(64).black());
            // }
        }
    }
    // println!("{}", "=".repeat(64).red());

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
            println!("{}", tl[0].tl);
            let out = &tl[verse];
            println!("{}", out.to_string().blue());
            
            if tl != tls.last().unwrap() {
                println!("{}", "-".repeat(64).black());
            }
        }
    }
    println!("{}", "=".repeat(64).red());
}

