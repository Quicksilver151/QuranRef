use crate::*;



pub fn edit(cfg: &mut Config) {
    println!("select a translation index to download");
    println!("======================================");
    
    let tl: Vec<Translation> = get_translations();
    tl.iter().for_each(|tl|println!("{}\t{}",tl.id, tl.name));
    
    println!("input a number: ");
    let number = get_number_input().unwrap();
    let mut tl_name = "unkown".to_owned();
    for i in tl {
        if i.id == number {
            tl_name = i.name;
            break;
        }
    }

    let selected_tl = Translation { id: number, name: tl_name };
    let quran = download_quran(&selected_tl);
    save_quran_data(quran);
    
    cfg.add_translation(selected_tl);
    dbg!(&cfg);
    cfg.save();
}

// input management
pub fn get_number_input() -> Result<u16, std::num::ParseIntError> {
    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    
    let trimmed = input_text.trim();
    trimmed.parse::<u16>()
}
