// std
use std::env;

// crates
pub use serde::{Serialize, Deserialize};
pub use signal_hook::{consts::SIGINT, iterator::Signals};

// include files
mod functions;
mod structs;

// use files
use functions::*;
use structs::*;

fn main() {
    // init
    handle_ctrlc();
    
    // load config
    let cfg_result: Result<Config, confy::ConfyError> = confy::load("quran-ref", None);
    let cfg =
        match cfg_result {
            Ok (cfg_result) => cfg_result,
            Err(cfg_result) => {
                println!("{}", cfg_result);
                Config::new()
            }
        };
    
    confy::store("quran-ref", None, cfg).unwrap_or_default();
    
    // fetch flags
    let args: Vec<String> = env::args().collect();
    let flag: Flag = match flag_parser::parse_args(args) {
        Ok(flag) => flag,
        Err(_flag) => return,
    };
    
    if flag.help {
        println!("{}",HELP_TEXT);
        return;
    }
    if flag.edit {
        todo!("edit function");
    } 
    if flag.index.chapter == 0 {
        println!("{}",HELP_TEXT);
        return;
    }
    if flag.arabic {
        todo!("display verses in arabic");
    }
       
    dbg!(flag);
    
}
