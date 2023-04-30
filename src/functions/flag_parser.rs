use crate::*;

// pub struct FlagErr;
#[derive(Debug)]
pub enum FlagErr {
    Invalid,
    Broken,
    Missing,

}

#[derive(Debug)]
pub struct Flag {
    pub help   : bool,
    pub edit   : bool,
    pub arabic : bool,
    pub index  : VerseIndex,
    pub endex  : VerseIndex,
}
impl Flag {
    fn new() -> Flag {
        Flag {
        help   : false,
        edit   : false,
        arabic : false,
        index  : VerseIndex::new(),
        endex  : VerseIndex::new(),
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
            println!("===INVALID FLAG ENTERED===\n");
            return Err(FlagErr::Invalid);
        }
        else if arg_vec[0] == '-' && arg_vec[1] == '-' {
            let argument = arg.strip_prefix("--").unwrap();
            match argument {
                "help"      =>  flag.help   = true,
                "edit"      =>  flag.edit   = true,
                "arabic"    =>  flag.arabic = true,
                _ => {
                    println!("===INVALID FLAG ENTERED===\n");
                    return Err(FlagErr::Invalid);
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
                        println!("==INVALID FLAG ENTERED===\n");
                        return Err(FlagErr::Invalid);
                    }
                }
            }
        }
        else if !arg.contains(':') {
            println!("Invalid verse format. ':' required");
            return Err(FlagErr::Broken);
        }
        else if flag.index.chapter == 0 {
            let splits : Vec<&str> = arg.split(':').collect();
            
            let (chapter_index, verse_index) = (splits[0],splits[1]);
            
            if chapter_index == "0" || verse_index == "0" {
                println!("Invalid verse entered. Cannot accept verse 0 or chapter 0.");
                return Err(FlagErr::Missing);
            }
            
            
            flag.index.chapter  = parse_num(chapter_index) .unwrap();
            flag.index.verse    = parse_num(verse_index)   .unwrap();
            flag.endex.chapter  = parse_num(chapter_index) .unwrap();
            flag.endex.verse    = parse_num(verse_index)   .unwrap();
            
        }
        else {
            let splits : Vec<&str> = arg.split(':').collect();
            let (chapter_index, verse_index) = (splits[0],splits[1]);
            
            if chapter_index == "0" || verse_index == "0" {
                println!("Invalid verse entered. cannot accept verse 0 or chapter 0.");
                return Err(FlagErr::Missing);
            }
            
            match chapter_index.parse::<u16>(){
                Ok(_) => {
                    flag.endex.chapter  = parse_num(chapter_index) .unwrap();
                    flag.endex.verse    = parse_num(verse_index)   .unwrap();
                },
                Err(_) => {
                    continue;
                }
                
            }
            
        }
    }
    
    Ok(flag)
}

fn parse_num(numstr:&str) -> Option<u16> {
    match numstr.parse::<u16>() {
        Ok(num) => Some(num),
        Err(error) => panic!("Invalid value entered, expected a non 0 integer.\n{}",error),
    }
}

fn parse_verse(verse_str: &str) -> Option<VerseIndex>{
    todo!()
}


