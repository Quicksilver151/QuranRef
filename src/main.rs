// std
use std::env;

// crates
pub use serde::{Serialize, Deserialize};
pub use colored::*;

// include files
mod utils;
mod models;
mod api;
// include!(concat!(env!("OUT_DIR"), "/db.rs"));

// use files
use utils::*;
use models::*;
use api::*;

#[allow(unused)]
fn main() {
    // init
    // handle_ctrlc();
    // dbg!(OKK);//buildscript test
    // load config
    let mut cfg = Config::load();
    config_init();
    
    // fetch flags
    let args: Vec<String> = env::args().collect();
    let flag: Flag = match flag_parser::parse_args(args) {
        Ok(flag) => flag,
        Err(_flag) => {println!("flag_err:{_flag:?}\n{}",HELP_TEXT); return;},
    };
    
    dbg!(&flag);
    // println!("VERSEEE=");
    // flag.verses.to_vec().iter().for_each(|x| println!("{}",x));
    
    
    // branch flags
    if flag.help {
        println!("{}",HELP_TEXT);
        return;
    }
    if flag.edit {
        print_translations();
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
    // Main code:
    for i in flag.verses.to_vec().iter() { 
        // TODO: FIX THE LINE SIZE DAMMIT
        println!("{:<5}|{}",format!("{}",i).bold(),"==========================================================".red());
        print_verse(i);
    }
        println!("{}","================================================================".red());
    
}


#[tokio::main]
pub async fn config_init(){
    let translations : Vec<Translation> = get_translation_list().await;
    let accepted = vec![20,131];
    let mut translations: Vec<Translation> = translations.into_iter().filter(|x| accepted.contains(&x.id)).collect();
    let cfg_result: Result<Config, confy::ConfyError> = confy::load("quran-ref", None);
    let mut cfg =
        match cfg_result {
            Ok (cfg_result) => cfg_result,
            Err(cfg_result) => {
                println!("{}", cfg_result);
                Config::default()
            }
        };
    cfg.translations.append(&mut translations);
    
    dbg!(&cfg);
    match confy::store("quran-ref", None, cfg) {
        Ok(_)  => println!("Stored data successfully"),
        Err(problem) => println!("Storing data failed due to {}", problem),
    }
}
#[tokio::main]
pub async fn print_verse(verse_index: &VerseIndex){
    
    let sahih : VerseData = get_verse_data(verse_index,  20).await;
    let clear : VerseData = get_verse_data(verse_index, 131).await;
    
    println!("Sahih International:\n{}\n{}\nDr.Mustafa Khattab, the Clear Quran:\n{}", sahih, "----------------------------------------------------------------".bright_black(),clear);
}


#[tokio::main] // actually this one featches translations not print em
pub async fn print_translations(){
    let translations = get_translation_list().await;
    translations.iter().for_each(|x|println!("{}",x));
}
