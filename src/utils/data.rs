use crate::*;

// TODO: fix to proper api data formats
pub struct QuranTranslation {
    pub chapters: Vec<Chapter>
}
pub struct Chapter{
    pub verses: Vec<Verse>
}



pub fn save_translation(content:&str, translation_name:&str){
    
    match save_to_data_dir(content, translation_name) {
        Ok(()) => println!("Translation downloaded successfully."),
        Err(err) => eprintln!("Error saving download: {}", err),
    }
}



// TODO: 
//  - Store translations
//      - Optimised format if possible
//  - Load translations
//  - Parse translations to verse list
//      - optimise wherever possible




