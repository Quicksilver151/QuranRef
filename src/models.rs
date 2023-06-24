use std::fmt::{Display, write};

use crate::*;


#[derive(Debug)]
pub enum VerseErr{Invalid}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct VerseIndex {
    pub chapter : u16,
    pub verse   : u16,
}
impl VerseIndex {
    pub fn from(index: &str) -> VerseIndex {
        let splits : Vec<&str> = index.split(':').collect();
        let (chapter_index, verse_index) = (splits[0],splits[1]);
        
        VerseIndex { chapter: parse_num(chapter_index).unwrap(), verse: parse_num(verse_index).unwrap()}
        
    } 
}

impl Display for VerseIndex {
    fn fmt(&self, w: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(w, "{}:{}", self.chapter, self.verse)
    }
}



#[derive(Default, Debug)]
pub struct VerseRange {
    pub index : VerseIndex,
    pub endex : VerseIndex,
}
impl VerseRange {
    pub fn from(verse_str: &str) -> Result<VerseRange, VerseErr> {
        
        let splits : Vec<&str> = verse_str.split('-').collect();
        let chapter: Vec<&str> = splits[0].split(':').collect();
        let (index, endex) = (splits[0], &format!("{}:{}", chapter[0], splits[1]));
        
        
        Ok(VerseRange {
            index: VerseIndex::from(index),
            endex: VerseIndex::from(endex),
        })
    }
    
    pub fn is_in_order(&self) -> bool{
        self.index.chapter < self.endex.chapter ||
        (
            self.index.chapter == self.endex.chapter &&
            self.index.verse <= self.endex.verse
        )
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

// tl:

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Translation {
    pub id: u16,
    pub name: String,
}

// quran data

#[derive(Debug, Default)]
pub struct Verse {
    pub text: String,
    pub tl_id : u16
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Chapter {
    pub verses: Vec<String>
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Quran {
    pub chapters: Vec<Vec<String>>,
    pub translation: Translation
}
impl Display for Quran {
    fn fmt(&self, w: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let quran_json = serde_json::to_string_pretty(self).unwrap();
        write!(w, "{}", quran_json)
    }
}





