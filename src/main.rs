// std
use std::env;

// crates
pub use serde::{Serialize, Deserialize};
pub use signal_hook::{consts::SIGINT, iterator::Signals};
pub use colored::*;

// include files
mod functions;
mod structs;
// include!(concat!(env!("OUT_DIR"), "/db.rs"));

// use files
use functions::*;
use structs::*;

fn main() {
    // init
    handle_ctrlc();
    // dbg!(OKK);//buildscript test
    // load config
    let cfg_result: Result<Config, confy::ConfyError> = confy::load("quran-ref", None);
    let cfg =
        match cfg_result {
            Ok (cfg_result) => cfg_result,
            Err(cfg_result) => {
                println!("{}", cfg_result);
                Config::default()
            }
        };
    dbg!(&cfg);
    
    confy::store("quran-ref", None, cfg).unwrap_or_default();
    
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
        todo!("edit function");
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
    println!("================================================================");
    for i in flag.verses.to_vec().iter() {
        print_data(i);
        println!("================================================================");
    }
    
}



#[tokio::main]
pub async fn print_data(verse_index: &VerseIndex){
    
    let sahih : VerseData = get_verse_data(verse_index,  20).await;
    let clear : VerseData = get_verse_data(verse_index, 131).await;
    
    
    
    
    println!("{}\n----------------------------------------------------------------\n{}", textwrap::wrap(&sahih.verse.translations[0].text, 64)[0].red() , clear.verse.translations[0].text);
}


