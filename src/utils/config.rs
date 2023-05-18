use crate::*;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub translations: Vec<u16>,
    pub arabic : bool,
}
impl Config {
    
    pub fn load() -> Config{
        let load_result: Result<Config, confy::ConfyError> = confy::load("quran-ref", None);
        match load_result {
            Ok (cfg) => cfg,
            Err(reason) => {
                println!("{}", reason);
                Config::default()
            }
        }
    }
    
    pub fn save(&self){
        match confy::store("quran-ref", None, self) {
            Ok(_) => println!("Saved config successfully!"),
            Err(reason) => println!("Err: saving failed due to: {}", reason),
        }
    }
}

// TODO:
//  list translations()
//     select translations()
//     save translation ids(config.rs)/download complete translations(download.rs)
//






