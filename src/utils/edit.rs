use crate::*;



pub fn edit(cfg: &mut Config) {
    
    let tls: Vec<Translation> = get_downloaded_translations_list();
    if tls.is_empty(){eprintln!("Err: {}","No downloaded translations found".red());return}
    
    // println!("Toggle selected downloaded translations by their index");
    
    new_buffer();
    clear_screen();
    println!("========================================================");
    println!("{:<5}|{:<10    }|name", "index", "status",);
    println!("-----|----------|---------------------------------------");
    for tl in tls.iter(){
        let status = match cfg.selected_tls.contains(tl){
            true  => "[selected]".green(),
            false => "".white(),
        };
        println!("{:<5}|{:<10}|{}",tl.id, status, tl.name)
    };
    println!("========================================================");
    println!("\nInput comma separated numbers: [eg: 20,131,270]");
    let numbers = get_number_list_input().unwrap_or_default();
    let mut selected_tls: Vec<Translation> = vec![];
    
    for number in numbers {
        for tl in tls.iter(){
            if tl.is_id(&number){
                selected_tls.append(&mut vec![tl.clone()]);
            }
        }
    }
    exit_buffer();
    if selected_tls.is_empty(){
        println!("No translations selected. No changes made");
        return;
    }
    cfg.selected_tls = selected_tls;
    dbg!(&cfg);
    cfg.save();
}

