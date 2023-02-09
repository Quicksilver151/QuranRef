use crate::*;

pub struct FlagErr;

#[derive(Debug)]
pub struct Flag {
    pub help   : bool,
    pub edit   : bool,
    pub arabic : bool,
    pub index  : VerseIndex,
    pub outdex : VerseIndex,
}
impl Flag {
    fn new() -> Flag {
        Flag {
        help   : false,
        edit   : false,
        arabic : false,
        index  : VerseIndex::new(),
        outdex : VerseIndex::new(),
        }
    }
}




pub const HELP_TEXT : &str =
"quran-ref
Display the verses of the quran in various english translations using references

Usage: quran-ref [OPTIONS] <START_CHAPTER:START_VERSE> <END_CHAPTER:END_VERSE>

eg: 
> quran-ref 21:12
> quran-ref 12:3 12:8
> quran-ref -a 3:23 4:10

OPTIONS:
    -h, --help          shows this help section
    -e, --edit          configure the program
    -a, --arabic        includes the arabic part
    
config contains island index
config is stored in ~/.config/quran-ref/";


pub fn parse_args(mut args: Vec<String>) -> Result<Flag, FlagErr> {
    // let mut args : Vec<String> = env::args().collect();
    args.reverse();
    args.pop();
    args.reverse();
    
    // println!("{:?}",args);
    let mut flag: Flag = Flag::new();
    
    for arg in args {
        let arg_vec: Vec<char> = arg.chars().collect::<Vec<char>>();
        if arg_vec.len() == 1 {
            println!("===INVALID FLAG ENTERED===\n\n{}", HELP_TEXT);
            return Err(FlagErr);
        }
        else if arg_vec[0] == '-' && arg_vec[1] == '-' {
            let argument = arg.strip_prefix("--").unwrap();
            match argument {
                "help"      =>  flag.help   = true,
                "edit"      =>  flag.edit   = true,
                "arabic"    =>  flag.arabic = true,
                _ => {
                    println!("===INVALID FLAG ENTERED===\n\n{}", HELP_TEXT);
                    return Err(FlagErr);
                }
            }
        }
        else if arg_vec[0] == '-' {
            for argchar in arg_vec {
                if argchar == '-' {
                    continue;
                }
                match argchar {
                    'h'     =>  flag.help   = true,
                    'e'     =>  flag.edit   = true,
                    'a'     =>  flag.arabic = true,
                    _ => {
                        println!("==INVALID FLAG ENTERED===\n\n{}", HELP_TEXT);
                        return Err(FlagErr);
                    }
                }
            }
        }
        else if flag.index.chapter == 0 {
            let splits : Vec<&str> = arg.split(':').collect();
            let (chapter_index, verse_index) = (splits[0],splits[1]);
            
            match chapter_index.parse::<u16>(){
                Ok(_) => {
                    flag.index.chapter  = chapter_index .parse::<u16>().unwrap_or(0);
                    flag.index.verse    = verse_index   .parse::<u16>().unwrap_or(0);
                    flag.outdex.chapter = chapter_index .parse::<u16>().unwrap_or(0);
                    flag.outdex.verse   = verse_index   .parse::<u16>().unwrap_or(0);
                },
                Err(_) => {
                    continue;
                }
            
            }
        }
        else {
            let splits : Vec<&str> = arg.split(':').collect();
            let (chapter_index, verse_index) = (splits[0],splits[1]);
            
            match chapter_index.parse::<u16>(){
                Ok(_) => {
                    flag.outdex.chapter = chapter_index .parse::<u16>().unwrap_or(0);
                    flag.outdex.verse   = verse_index   .parse::<u16>().unwrap_or(0);
                },
                Err(_) => {
                    continue;
                }
                
            }
            
        }
    }
    
    Ok(flag)
}
