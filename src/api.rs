use crate::*;


// Structs
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Translation {
    pub id: u32,
    pub resource_id: u32,
    pub text: String,
}
impl std::fmt::Display for Translation {
    fn fmt(&self, w: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(w, "{}. ({}) {}",self.id, self.resource_id, self.text)
    }
}

fn remove_section(s: &str, start: usize, end: usize) -> String {
    let mut result = String::new();
    result.push_str(&s[..start]);
    result.push_str(&s[end..]);
    result
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct VerseData {
    pub verse: Verse,
}
impl std::fmt::Display for VerseData {
    fn fmt(&self, w: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        
        let mut text = self.verse.translations[0].text.to_string();
        loop{
            match text.find("<sup"){
                Some(found) => text = remove_section(&text, found, text.find("</sup>").unwrap_or(found)+6),
                None => break,
            };
        }
        let wraped_text = textwrap::fill(&text, 64);
        write!(w, "{}", wraped_text.blue())
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Verse {
    pub id: u16,
    pub verse_number: u16,
    pub verse_key: String,
    pub hizb_number: u16,
    pub rub_el_hizb_number: u16,
    pub ruku_number: u16,
    pub manzil_number: u16,
    pub sajdah_number: Option<u16>,
    pub page_number: u16,
    pub juz_number: u16,
    pub translations: Vec<Translation>,
}



pub async fn get_verse_data(verse_index: &VerseIndex, translation: u16) -> VerseData {
    let body = reqwest::get(format!("https://api.quran.com/api/v4/verses/by_key/{}?language=en&translations={}",verse_index, translation))
        .await.unwrap()
        .text()
        .await.unwrap();
    
    serde_json::from_str(&body).unwrap_or_default()
}

// TODO:
pub async fn get_translation_data(verse_index: &VerseIndex, translation: u16) -> Translation {
    let body = reqwest::get(format!("https://api.quran.com/api/v4/verses/by_key/{}?language=en&translations={}",verse_index, translation))
        .await.unwrap()
        .text()
        .await.unwrap();
    
    // serde_json::from_str(&body).unwrap_or_default();
    todo!()
}
