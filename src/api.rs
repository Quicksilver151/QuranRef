use crate::*;

// =====
// Verse
// =====

// structs
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TranslationText {
    pub id: u32,
    pub resource_id: u32,
    pub text: String,
}

impl std::fmt::Display for TranslationText {
    fn fmt(&self, w: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(w, "{}. ({}) {}", self.id, self.resource_id, self.text)
    }
}

fn remove_section(s: &str, start: usize, end: usize) -> String {
    let mut result = String::new();
    result.push_str(&s[..start]);
    result.push_str(&s[end..]);
    result
}
pub fn remove_sup_tag(text: String) -> String {
    let mut text = text;
    let mut failsafe = 50;
    
    while let Some(found) = text.find("<sup"){
        text = remove_section(&text, found, text.find("</sup>").unwrap_or(found) + 6);
        failsafe -= 1;
        if failsafe < 0 {
            break;
        }
    }
    text
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct VerseData {
    verse: Verse,
}
impl std::fmt::Display for VerseData {
    fn fmt(&self, w: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let text = self.verse.translations[0].text.to_string();
        let text = remove_html_tags(&remove_sup_tag(text));
        let wraped_text = textwrap::fill(&text, 64);
        write!(w, "{}", wraped_text.blue())
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Verse {
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
    pub translations: Vec<TranslationText>,
}



// ====================
// ApiTranslation Names
// ====================

// structs
#[derive(Default, Debug, Serialize, Deserialize)]
struct TranslatedName {
    pub name: String,
    pub language_name: String,
}
#[derive(Default, Debug, Serialize, Deserialize)]
struct TranslationData {
    id: u16,
    name: String,
    author_name: String,
    slug: Option<String>,
    language_name: String,
    translated_name: TranslatedName,
}
impl std::fmt::Display for TranslationData {
    fn fmt(&self, w: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let splitname: Vec<&str> = self.language_name.split(',').collect();
        write!(
            w,
            "{0}.\t{1:<11}\t{2}",
            self.id.to_string().green(),
            splitname[0].red(),
            self.translated_name.name.blue()
        )
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct ApiTranslations {
    pub translations: Vec<TranslationData>,
}

#[tokio::main]
pub async fn get_translation_list() -> Vec<Translation> {
    let data = match reqwest::get("https://api.quran.com/api/v4/resources/translations").await {
        Ok(data) => data,
        Err(_) => panic!("no network connection"),
    };
    
    let body = match data.text().await {
        Ok(body) => body,
        Err(problem) => panic!("failed to extract text: {}", problem),
    };
    
    let api_translations = serde_json::from_str::<ApiTranslations>(&body)
        .unwrap()
        .translations;
    
    api_translations
        .into_iter()
        .map(|x| Translation {
            name: x.name,
            id: x.id,
        })
        .collect()
}

// =================
// Full translations
// =================

pub async fn fetch_chapter(translation: &Translation, chapter_number: u16) -> Vec<String> {
    // api structs:
    #[derive(Default, Debug, Serialize, Deserialize)]
    struct Filters {
        resource_id: u16,
        chapter_number: String,
    }
    #[derive(Default, Debug, Serialize, Deserialize)]
    struct ChapterMeta {
        translation_name: String,
        author_name: String,
    }
    #[derive(Default, Debug, Serialize, Deserialize)]
    struct Translation {
        resource_id: u16,
        text: String,
    }
    #[derive(Default, Debug, Serialize, Deserialize)]
    struct ApiChapter {
        translations: Vec<Translation>,
        meta: ChapterMeta,
    }
    
    let url = format!(
        "https://api.quran.com/api/v4/quran/translations/{}?chapter_number={}",
        translation.id, chapter_number
    );
    let data = match reqwest::get(url).await {
        Ok(data) => data,
        Err(_) => panic!("no network connection"),
    };
    
    let body = match data.text().await {
        Ok(body) => body,
        Err(problem) => panic!("failed to extract text: {}", problem),
    };
    
    let chapter = serde_json::from_str::<ApiChapter>(&body)
        .unwrap()
        .translations;
    
    let mut verses: Vec<String> = vec![chapter_number.to_string()];
    verses.append(&mut chapter.into_iter().map(|x| x.text).collect());
    
    verses
        .into_iter()
        .map(|x| remove_html_tags(&remove_sup_tag(x)))
        .collect()
}

pub async fn fetch_quran(translation: &Translation) -> Quran {
    let mut quran = Quran::default();
    
    quran.chapters.append(&mut vec![vec!["-".to_owned()]]);
    
    for i in 1..115 {
        println!("{} {}", "downloading chapter:".yellow(), i);
        let chapter = fetch_chapter(translation, i).await;
        quran.chapters.append(&mut vec![chapter])
    }
    
    quran.translation = Translation {
        id: translation.id,
        name: translation.name.to_owned(),
    };
    quran
}

#[tokio::main]
pub async fn download_quran(translation: &Translation) -> Quran {
    fetch_quran(translation).await
}



// DEPRECIATED ONLINE FETCH CODE
// async fn get_verse_data(verse_index: &VerseIndex, translation: u16) -> VerseData {
//     let body = reqwest::get(format!(
//         "https://api.quran.com/api/v4/verses/by_key/{}?language=en&translations={}",
//         verse_index, translation
//     ))
//     .await
//     .unwrap()
//     .text()
//     .await
//     .unwrap();
//     
//     match serde_json::from_str(&body) {
//         Ok(verse_data) => verse_data,
//         Err(problem) => {
//             println!("Failed due to: {}\nselecting default values", problem);
//             VerseData::default()
//         }
//     }
// }



