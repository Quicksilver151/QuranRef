// std
use std::env;

// crates
pub use serde::{Serialize, Deserialize};
pub use colored::*;
pub use directories::*;

// include files
mod utils;
mod models;
mod api;

// use files
use utils::*;
use models::*;
use api::*;


#[allow(unused)]
fn main() {
    // init
    // load config & data
    let downloaded_tls = get_downloaded_translations_list();
    let mut cfg = Config::load();
    
    // fetch flags
    let args: Vec<String> = env::args().collect();
    let flag: Flag = match flag_parser::parse_args(args) {
        Ok(flag) => flag,
        Err(_flag) => {println!("flag_err:{_flag:?}\n{}",HELP_TEXT); return;},
    };
    
    dbg!(&flag);
    
    
    // branch flags
    if flag.help {
        println!("{}",HELP_TEXT);
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
        println!("{}",HELP_TEXT);
        return;
    }
    if flag.verses.is_in_order(){
        println!("fetching verses:");
    }else{
        todo!("handle verses in reverse order");
    }
    //Main code:
    //  if .local has cfg selected translation:
    //      fetch local
    //  else:
    //      fetch online (done!) (remove feature)
    dbg!(&downloaded_tls, &cfg.current_tl);
    if cfg.selected_tls.is_empty() {
        eprintln!("{}", "Download some translations with -d and select them with -e".red())
    }
    for tl in cfg.selected_tls.iter() {
        let quran = load_downloaded_translation(tl);
        show_verses(&quran, &flag.verses);
    }
    // for index in flag.verses.to_vec().iter() { 
    //     // TODO: FIX THE LINE SIZE DAMMIT
    //     println!("{:<5}|{}",format!("{}",index).bold(),"==========================================================".red());
    //     print_verse(index);
    // }
    // println!("{}","================================================================".red());
    
}

pub fn show_verses(quran: &Quran, verse_range: &VerseRange) {
    
    let verses = quran.fetch_verses(verse_range);
    for verse in verses {
        println!("{}",verse);
    }
    
    
}

#[tokio::main]
pub async fn print_verse(verse_index: &VerseIndex){

    let sahih : VerseData = get_verse_data(verse_index,  20).await;
    let clear : VerseData = get_verse_data(verse_index, 131).await;
    
    println!("Sahih International:\n{}\n{}\nDr.Mustafa Khattab, the Clear Quran:\n{}", sahih, "----------------------------------------------------------------".bright_black(),clear);
}


#[tokio::main]
pub async fn download_quran(translation: &Translation)-> Quran {
    
    fetch_quran(translation).await
    
    
}


