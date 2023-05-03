// std
use std::{env, ptr::NonNull};


// crates
pub use serde::{Serialize, Deserialize};
pub use signal_hook::{consts::SIGINT, iterator::Signals};
// use hyper::Client;
use tokio::sync;
use reqwest::get;
use serde_json;

include!(concat!(env!("OUT_DIR"), "/db.rs"));
// include files
mod functions;
mod structs;

// use files
use functions::*;
use structs::*;

fn main() {
    // init
    handle_ctrlc();
    dbg!(OKK);
    // load config
    let cfg_result: Result<Config, confy::ConfyError> = confy::load("quran-ref", None);
    let cfg =
        match cfg_result {
            Ok (cfg_result) => cfg_result,
            Err(cfg_result) => {
                println!("{}", cfg_result);
                Config::new()
            }
        };
    
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
    
    
    println!("================================================================");
    for i in flag.verses.to_vec().iter() {
        get_data(i);
        println!("================================================================");
    }
    
    // for i in 1..8{
    //     get_data(&format!("1:{}",i));
    // }
    
    // branch flags
    if flag.help {
        println!("{}",HELP_TEXT);
        return;
    }
    if flag.edit {
        todo!("edit function");
    } 
    if flag.verses.index.chapter == 0 {
        println!("{}",HELP_TEXT);
        return;
    }
    if flag.arabic {
        todo!("display verses in arabic");
    }
    
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Translation {
    id: u32,
    resource_id: u32,
    text: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Data {
    verse: Verse,
}
#[derive(Default, Debug, Serialize, Deserialize)]
struct Verse {
    id: u16,
    verse_number: u16,
    verse_key: String,
    hizb_number: u16,
    rub_el_hizb_number: u16,
    ruku_number: u16,
    manzil_number: u16,
    sajdah_number: Option<u16>,
    page_number: u16,
    juz_number: u16,
    translations: Vec<Translation>,
}

#[tokio::main]
async fn get_data(verse_index: &VerseIndex){
    
    // sahih international
    let body = reqwest::get(format!("https://api.quran.com/api/v4/verses/by_key/{}?language=en&translations=20",verse_index))
        .await.unwrap()
        .text()
        .await.unwrap();
    
    let sahih: Data = serde_json::from_str(&body).unwrap_or_default();
    
    let body = reqwest::get(format!("https://api.quran.com/api/v4/verses/by_key/{}?language=en&translations=131",verse_index))
        .await.unwrap()
        .text()
        .await.unwrap();
    
    let clear: Data = serde_json::from_str(&body).unwrap_or_default();
    
    println!("{}\n----------------------------------------------------------------\n{}", sahih.verse.translations[0].text, clear.verse.translations[0].text);
    
}


