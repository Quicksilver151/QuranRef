use crate::*;



pub fn edit() {
    println!("select a translation index to download");
    println!("======================================");
    
    let tl: Vec<(String, u16)> = get_translations();
    tl.iter().for_each(|tl|println!("{}\t{}",tl.1, tl.0));
    
    println!("input a number: ");
    let number = get_number_input().unwrap();
    let mut tl_name = "unkown".to_owned();
    for i in tl {
        if i.1 == number {
            tl_name = i.0;
            break;
        }
    }

    let quran = download_quran(Translation { id: number, name: tl_name });
    save_quran_data(quran);
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
