use crate::*;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub current_tl: Translation,
    pub selected_tls: Vec<Translation>,
}
impl Config {
    pub fn load() -> Config {
        let load_result: Result<Config, confy::ConfyError> = confy::load("quran-ref", "config");
        match load_result {
            Ok(cfg) => cfg,
            Err(reason) => {
                println!(
                    "Failed to load config due to: {}\nCreating new config with defaults",
                    reason
                );
                Config::default()
            }
        }
    }
    
    pub fn save(&self) {
        match confy::store("quran-ref", "config", self) {
            Ok(_) => println!("Saved config successfully!"),
            Err(reason) => println!("Err: saving failed due to: {}", reason),
        }
    }
}

//  TODO:
//  list translations()x
//      select translations()x
//          sort translations() / filter translations()
//      save translation ids(config.rs)x / download complete translations(download.rs)x
//
