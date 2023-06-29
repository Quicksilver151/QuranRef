use crate::*;



pub fn edit(cfg: &mut Config) {
    println!("Select downloaded translations by their number");
    println!("==============================================");
    
    let tls: Vec<Translation> = get_downloaded_translations_list();
    
    tls.iter().for_each(|tl|println!("{}\t{}",tl.id, tl.name));
    
    println!("input comma separated numbers: [eg: 20,131,270]");
    let numbers = get_number_list_input().unwrap();
    let mut selected_tls: Vec<Translation> = vec![];

    for number in numbers {
        for tl in tls.iter(){
            if tl.is_id(&number){
                selected_tls.append(&mut vec![tl.clone()]);
            }
        }
    }
    
    cfg.selected_tls = selected_tls;
    dbg!(&cfg);
    cfg.save();
}

