use crate::*;


pub fn save_translation(content:&str, translation_name:&str){
    
    match save_to_data_dir(content, translation_name) {
        Ok(()) => println!("Translation downloaded successfully."),
        Err(err) => eprintln!("Error saving download: {}", err),
    }
}


