use crate::*;


use clap::{Parser};

#[derive(Parser, Debug)]
#[command(
    name = "quran-ref",
    about = "Display the verses of the Quran in various English translations using references",
    override_usage = r#"
  quran-ref [OPTIONS] <START_CHAPTER>:<START_VERSE> <END_CHAPTER>:<END_VERSE>
  quran-ref [OPTIONS] <START_CHAPTER>:<START_VERSE>-<END_VERSE>"#,
    after_help = r#"
Display the verses of the Quran in various English translations using references.

EXAMPLES:
  quran-ref 21:12
      (prints verse 21:12)

  quran-ref 12:3 12:8
      (prints verses in range [12:3 .. 12:8])

  quran-ref -a 3:23-28
      (prints verses [3:23 .. 3:28] with arabic)

  quran-ref -a 3:10 3:14
      (prints verses [3:10 .. 3:14] with arabic)

On Linux:
  config is stored in ~/.config/quran-ref/
  data and translations are stored in ~/.local/share/quran-ref/
"#
)]
pub struct Cli {
    /// Select the translations to display
    #[arg(short, long)]
    pub edit: bool,

    /// Download translations for offline use
    #[arg(short, long)]
    pub download: bool,

    /// Include Arabic text (WIP)
    #[arg(short = 'a', long)]
    pub arabic: bool,

    /// Output text only, without formatting
    #[arg(short, long)]
    pub raw: bool,

    /// Verse reference (eg: 2:1, 3:12-15)
    ///
    /// Examples:
    ///   21:12
    ///   12:3 12:8
    ///   3:23-28
    #[arg(value_name = "VERSE", num_args = 1..=2)]
    pub verses: Vec<String>,
}


use anyhow::{Result, bail};

impl Cli {
    pub fn verse_range(&self) -> Result<VerseRange, VerseErr> {
        match self.verses.as_slice() {
            [single] => {
                if single.contains('-') {
                    Ok(VerseRange::from(single).unwrap())
                } else {
                    let idx = VerseIndex::from(single)?;
                    Ok(VerseRange {
                        index: idx,
                        endex: idx,
                    })
                }
            }
            [start, end] => Ok(VerseRange {
                index: VerseIndex::from(start)?,
                endex: VerseIndex::from(end)?,
            }),
            _ => Err(VerseErr::Invalid),
        }
    }
}

// // pub struct FlagErr;
// #[derive(Debug)]
// pub enum FlagErr {
//     Invalid,
//     Broken,
// }
//
// #[derive(Default, Debug)]
// pub struct Flag {
//     pub help: bool,
//     pub edit: bool,
//     pub download: bool,
//     pub arabic: bool,
//     pub verses: VerseRange,
// }
//
// pub const HELP_TEXT: &str = "quran-ref
// Display the verses of the quran in various english translations using references
//
//
// USAGE:  quran-ref [OPTIONS] <START_CHAPTER:START_VERSE> <END_CHAPTER:END_VERSE>
//             or
//         quran-ref [OPTIONS] <START_CHAPTER:START_VERSE>-<END_VERSE>
//
// OPTIONS:
//     -h, --help          shows this help section
//     -e, --edit          select the translations to display
//     -d, --download      download translations to use the program offline
//     -a, --arabic        includes the arabic part (WIP)
//
// EXAMPLES: 
// $ quran-ref 21:12
//     (prints verse 21:12)
//
// $ quran-ref 12:3 12:8 
//     (prints verses in range [12:3, 12:4, 12:5, 12:6, 12:7, 12:8])
//
// $ quran-ref -a 3:23-28
//     (prints verses in range [3:23, 3:24, 3:25, 3:26, 3:27, 3:28] with arabic)
//
// $ quran-ref -a 3:10 3:14
//     (prints verses in range [3:10, 3:11, 3:12, 3:13, 3:14] with arabic)
//
//
// on linux:
// config is stored in ~/.config/quran-ref/
// data and translations are stored in ~/.local/share/quran-ref/";
//
// pub fn parse_args(mut args: Vec<String>) -> Result<Flag, FlagErr> {
//     // let mut args : Vec<String> = env::args().collect();
//     args.reverse();
//     args.pop();
//     args.reverse();
//
//     // println!("{:?}",args);
//     let mut flag: Flag = Flag::default();
//
//     for arg in args {
//         let arg_vec: Vec<char> = arg.chars().collect::<Vec<char>>();
//         if arg_vec.len() == 1 {
//             println!("===INVALID FLAG ENTERED===\n");
//             return Err(FlagErr::Invalid);
//         } else if arg_vec[0] == '-' && arg_vec[1] == '-' {
//             let argument = arg.strip_prefix("--").unwrap();
//             match argument {
//                 "help"      => flag.help     = true,
//                 "edit"      => flag.edit     = true,
//                 "download"  => flag.download = true,
//                 "arabic"    => flag.arabic   = true,
//                 _           => {println!("===INVALID FLAG ENTERED===\n"); return Err(FlagErr::Invalid);}
//             }
//         } else if arg_vec[0] == '-' {
//             for argchar in arg_vec {
//                 if argchar == '-' {
//                     continue;
//                 }
//                 match argchar {
//                     'h'     => flag.help     = true,
//                     'e'     => flag.edit     = true,
//                     'd'     => flag.download = true,
//                     'a'     => flag.arabic   = true,
//                     _ => {
//                         println!("==INVALID FLAG ENTERED===\n");
//                         return Err(FlagErr::Invalid);
//                     }
//                 }
//             }
//         } else if !arg.contains(':') {
//             println!("Invalid verse format. ':' required");
//             return Err(FlagErr::Broken);
//
//         // index-endex range
//         } else if arg.contains('-') {
//             flag.verses = VerseRange::from(&arg).unwrap();
//
//         // index
//         } else if flag.verses.index.chapter == 0 {
//             flag.verses.index = VerseIndex::from(&arg);
//             flag.verses.endex = VerseIndex::from(&arg);
//
//         // endex
//         } else {
//             flag.verses.endex = VerseIndex::from(&arg);
//         }
//     }
//
//     Ok(flag)
// }
