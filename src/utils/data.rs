
use crate::*;


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

pub fn load_downloaded_translation(translation: &Translation) -> Quran {
    
    // Get the project directories
    let project_dirs = ProjectDirs::from("", "", "quran-ref").unwrap();
    
    // Get the path to the data directory
    let mut tl_file_path = project_dirs.data_dir().to_path_buf();
    let tl_name = format!("{} {}", translation.id, translation.name);
    tl_file_path.push(tl_name);
    
    let quran_json = match std::fs::read_to_string(tl_file_path){
        Ok(x) => x,
        Err(_) => {eprintln!("{}","No translations downloaded\nRun program with -d flag to download translations".red()); panic!();},
    };
    
    let quran: Quran = serde_json::from_str(&quran_json).unwrap();
    
    quran
    
}

pub fn get_downloaded_translations_list() -> Vec<Translation> {
    // Get the project directories
    let project_dirs = ProjectDirs::from("", "", "quran-ref").unwrap();
    
    // Get the path to the data directory
    let data_dir = project_dirs.data_dir();
    
    let file_names: Vec<String> = match data_dir.read_dir() {
        Ok(path) => path.into_iter().map(|x|x.unwrap().file_name().to_str().unwrap().to_string()).collect(),
        Err(err) => {println!("Translation downlods folder missing\nErr: {}",err.to_string().red());vec![]}
    };
    
    // parse name to translation struct
    let mut tl :Vec<Translation>= vec![];
    for file in file_names { 
        let (strid, strname) = file.split_once(' ').unwrap_or_default();
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


pub fn download_translation() {
    println!("select a translation index to download");
    println!("======================================");
    
    let tl: Vec<Translation> = get_translation_list();
    
    tl.iter().for_each(|tl|println!("{}\t{}",tl.id, tl.name));
    
    println!("input a number: ");
    let number = get_number_input().unwrap();
    let mut tl_name = "unkown".to_owned();
    for i in tl {
        if i.id == number {
            tl_name = i.name;
            break;
        }
    }
    
    let selected_tl = Translation { id: number, name: tl_name };
    let quran = download_quran(&selected_tl);
    save_quran_data(quran);
}





