use crate::*;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Translation {
    pub id: u32,
    pub resource_id: u32,
    pub text: String,
}



#[derive(Default, Debug, Serialize, Deserialize)]
pub struct VerseData {
    pub verse: Verse,
}
impl std::fmt::Display for VerseData {
    fn fmt(&self, w: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let wraped_text = textwrap::fill(&self.verse.translations[0].text, 64);
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
