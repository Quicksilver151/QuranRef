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



#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Translation {
    id: u32,
    resource_id: u32,
    text: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct VerseData {
    verse: Verse,
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Verse {
    id: u16,
    verse_number: u16,
    verse_key: String,
    hizb_number: u16,
    rub_el_hizb_number: u16,
    ruku_number: u16,
    manzil_number: u16,
    sajdah_number: Option<u16>,
    page_number: u16,
    juz_number: u16,
    translations: Vec<Translation>,
}

#[tokio::main]
pub async fn get_data(verse_index: &VerseIndex){
    
    // sahih international
    
    let sahih = get_verse_data(verse_index,  20).await;
    let clear = get_verse_data(verse_index, 131).await;
    
    println!("{}\n----------------------------------------------------------------\n{}", sahih.verse.translations[0].text, clear.verse.translations[0].text);
}


pub async fn get_verse_data(verse_index: &VerseIndex, translation: u16) -> VerseData{
    let body = reqwest::get(format!("https://api.quran.com/api/v4/verses/by_key/{}?language=en&translations={}",verse_index, translation))
        .await.unwrap()
        .text()
        .await.unwrap();
    
    serde_json::from_str(&body).unwrap_or_default()
}


