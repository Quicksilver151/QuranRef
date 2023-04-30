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
    pub fn from(index: &str) -> VerseIndex {
        let splits : Vec<&str> = index.split(':').collect();
        let (chapter_index, verse_index) = (splits[0],splits[1]);
        
        VerseIndex { chapter: parse_num(chapter_index).unwrap(), verse: parse_num(verse_index).unwrap()}
        
    } 
}

#[derive(Debug)]
pub enum VerseErr{NotFound, Invalid, LimitExceeded, Empty}

#[derive(Debug)]
pub struct VerseRange {
    pub index : VerseIndex,
    pub endex : VerseIndex,
}
impl VerseRange {
    pub fn new() -> VerseRange {
        VerseRange { index: VerseIndex::new(), endex: VerseIndex::new() }
    }
    pub fn from(verse_str: &str, limit: u16) -> Result<VerseRange, VerseErr> {
        let splits : Vec<&str> = verse_str.split('-').collect();
        
        let (index, endex) = (splits[0],splits[1]);
        
        Ok(VerseRange {
            index: VerseIndex::from(index),
            endex: VerseIndex::from(endex),
        })
    }
    
    pub fn is_in_order(&self) -> bool{
        true
    }
}
pub fn parse_num(numstr: &str) -> Result<u16, VerseErr> {
    numstr.parse::<u16>().map_err(|e|VerseErr::Invalid)
}



