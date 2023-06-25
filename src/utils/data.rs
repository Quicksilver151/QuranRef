
use crate::*;

// TODO: fix to proper api data formats
// pub struct QuranTranslation {
//     pub chapters: Vec<Chapter>
// }
// pub struct Chapter{
    // pub verses: Vec<Verse>
// }



pub fn save_translation(content:&str, translation_name:&str){
    
    match save_to_data_dir(content, translation_name) {
        Ok(()) => println!("Translation downloaded successfully."),
        Err(err) => eprintln!("Error saving download: {}", err),
    }
}

pub fn save_quran_data(quran: Quran) {
    match save_to_data_dir(&format!("{}",quran), &format!("{} {}", &quran.translation.id, &quran.translation.name)) {
        Ok(()) => println!("Translation downloaded successfully."),
        Err(err) => eprintln!("Error saving download: {}", err),
    }  
}

pub fn load_quran_data() -> Quran {
    todo!();
}
pub fn list_downloaded_translations() -> Vec<Translation> {
    // Get the project directories
    let project_dirs = ProjectDirs::from("", "", "quran-ref").unwrap();
    
    // Get the path to the data directory
    let data_dir = project_dirs.data_dir();
    
    let file_names: Vec<String> = data_dir
        .read_dir()
        .expect("Read the contents of the local folder")
        .map(|x| x.unwrap().file_name().to_str().unwrap().to_string()).collect();
    
    // parse name to translation struct
    let mut tl :Vec<Translation>= vec![];
    for file in file_names { 
        let (strid, strname) = file.split_once(' ').unwrap();
        let id = match strid.parse::<u16>() {
            Ok(id) => id,
            Err(_) => continue,
        };
        let name = strname.to_owned();
        let translation = Translation{id, name};
        tl.append(&mut vec![translation]);
    }
    tl
}

// TODO: 
//  - Store translations
//      - Optimised format if possible
//  - Load translations
//  - Parse translations to verse list
//      - optimise wherever possible




