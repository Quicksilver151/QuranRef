// std
use std::env;


// crates
pub use serde::{Serialize, Deserialize};
pub use signal_hook::{consts::SIGINT, iterator::Signals};
// use hyper::Client;
use tokio::sync;
use reqwest::get;
use serde_json;

include!(concat!(env!("OUT_DIR"), "/db.rs"));
// include files
mod functions;
mod structs;

// use files
use functions::*;
use structs::*;

fn main() {
    // init
    handle_ctrlc();
    dbg!(OKK);
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
        Err(_flag) => {println!("flag_err:{_flag:?}\n{}",HELP_TEXT); return;},
    };
    
    dbg!(&flag);
    
    
    
    
    
    
    
    get_data();
    
    
    // branch flags
    if flag.help {
        println!("{}",HELP_TEXT);
        return;
    }
    if flag.edit {
        todo!("edit function");
    } 
    if flag.verses.index.chapter == 0 {
        println!("{}",HELP_TEXT);
        return;
    }
    if flag.arabic {
        todo!("display verses in arabic");
    }
    
}


#[tokio::main]
async fn get_data(){
    // testing
    // let client = Client::new();
    // 
    // // Parse an `http::Uri`...
    // let uri = "http://api.quran.com/api/v4/verses/by_key/2:225?language=en&translations=823".parse().unwrap();
    // 
    // // Await the response...(not)
    // let resp = client.get(uri).await;
    // 
    // println!("Response: {:?}", resp.unwrap());
    
    let body = reqwest::get("http://api.quran.com/api/v4/verses/by_key/2:225?language=en&translations=823")
        .await.unwrap()
        .text()
        .await.unwrap();
    
    
    
    println!("{}", body);
    
}


