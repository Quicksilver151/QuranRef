use std::fmt::Display;

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
pub enum VerseErr{NotFound, Invalid, LimitExceeded, Empty}

#[derive(Debug, Serialize, Deserialize)]
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
    pub fn from_range(index: &str, range: u16) -> VerseIndex {
        let splits : Vec<&str> = index.split(':').collect();
        let (chapter_index, verse_index) = (splits[0],splits[1]);
        
        VerseIndex { chapter: parse_num(chapter_index).unwrap(), verse:  parse_num(verse_index).unwrap().max(range)}
        
    } 
}
impl Display for VerseIndex {
    fn fmt(&self, w: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(w, "{}:{}", self.chapter, self.verse)
    }
}



#[derive(Debug)]
pub struct VerseRange {
    pub index : VerseIndex,
    pub endex : VerseIndex,
}
impl VerseRange {
    pub fn new() -> VerseRange {
        VerseRange { index: VerseIndex::new(), endex: VerseIndex::new() }
    }
    pub fn from(verse_str: &str) -> Result<VerseRange, VerseErr> {
        let splits : Vec<&str> = verse_str.split('-').collect();
        
        let (index, range) = (splits[0], parse_num(splits[1]).unwrap());
        
        Ok(VerseRange {
            index: VerseIndex::from(index),
            endex: VerseIndex::from_range(index, range),
        })
    }
    
    pub fn is_in_order(&self) -> bool{
        true
    }
    
    pub fn to_vec(&self) -> Vec<VerseIndex>{
        let mut verse_indexes: Vec<VerseIndex> = vec![];
        
        for i in self.index.verse..(self.endex.verse+1) {
            verse_indexes.append(&mut vec![VerseIndex {chapter:self.index.chapter, verse: i}])
        }
        
        verse_indexes
    }
    
}
pub fn parse_num(numstr: &str) -> Result<u16, VerseErr> {
    numstr.parse::<u16>().map_err(|_|VerseErr::Invalid)
}



