// std
use std::env;

// crates
pub use serde::{Serialize, Deserialize};
pub use colored::*;

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
    // load config
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
    let translation: Translation = Translation { id: 131, name: "Dr. Mustafa Khattab, the Clear Quran".to_string() };
    if cfg.translations.contains(&translation){
        print!("okk")
    }
    else{
        print!("noooo{:?}",cfg.translations)
    }
    for i in flag.verses.to_vec().iter() { 
        // TODO: FIX THE LINE SIZE DAMMIT
        println!("{:<5}|{}",format!("{}",i).bold(),"==========================================================".red());
        print_verse(i);
    }
    println!("{}","================================================================".red());
    
}
pub fn print_verses(verse_range: &VerseRange, translation: &Translation) {

}



#[tokio::main]
pub async fn config_init(){
    let translations : Vec<Translation> = get_translation_list().await;
    let accepted = vec![20,131];
    let mut translation_ids: Vec<Translation> = translations.into_iter().filter(|x| accepted.contains(&x.id)).collect();// REWORK ALL Of DIS
    let cfg_result: Result<Config, confy::ConfyError> = confy::load("quran-ref", None);
    let mut cfg =
        match cfg_result {
            Ok (cfg_result) => cfg_result,
            Err(cfg_result) => {
                println!("{}", cfg_result);
                Config::default()
            }
        };
    cfg.translations.append(&mut translation_ids);
    
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
    let tl: Vec<Translation> = get_translation_list().await;
    tl.iter().for_each(|tl|println!("{}\t{}",tl.id, tl.name));
}
#[tokio::main] // actually this one featches translations not print em
pub async fn get_translations() -> Vec<Translation>{
    get_translation_list().await
}

#[tokio::main]
pub async fn download_quran(translation: &Translation)-> Quran {
    
    fetch_quran(translation).await
    
    
}


