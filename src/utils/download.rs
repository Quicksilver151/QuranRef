use crate::*;


pub fn save_translation(content:&str, filename:&str){
    
    match save_to_data_dir(content, filename) {
        Ok(()) => println!("Translation downloaded successfully."),
        Err(err) => eprintln!("Error saving download: {}", err),
    }
}


