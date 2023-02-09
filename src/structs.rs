use crate::*;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub translations: Vec<u8>,
}
impl Config {
    pub fn new() -> Config {
        Config { translations : vec![] }
    }
}

#[derive(Debug)]
pub struct VerseIndex {
    pub chapter : u16,
    pub verse   : u16,
}
impl VerseIndex {
    pub fn new() -> VerseIndex {
        VerseIndex { chapter: 0, verse: 0 }
    }
}





