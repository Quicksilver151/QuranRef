# QuranRef

Display the verses of the Quran in various English translations using references

- ~~(currently outputs both translations from **Sahih International** & **Dr.Mustafa Khattab, the Clear Quran**)~~

- ~~(Currently only works when online)~~

## Installation

- run `cargo install quran-ref`

## Initial Setup

- Download a translation:

```
$ quran-ref -d
```

- Select a translation / select multiple translations:

```
$ quran-ref -e
```

Selecting multiple translations require multiple translations to be downloaded
If any problem occurs, rename or delete your config file/data file.
File paths are given in --help

## Usage

`$ quran-ref` `[OPTIONS]` `<START_CHAPTER:START_VERSE>` `<END_CHAPTER:END_VERSE>`

or

`$ quran-ref` `[OPTIONS]` `<START_CHAPTER:START_VERSE>-<END_VERSE>`

```
OPTIONS:
    -h, --help          shows this help section
    -e, --edit          select the translations to display
    -d, --download      download translations to use the program offline
    -a, --arabic        includes the arabic part (yet to be implemented)

EXAMPLES:

$ quran-ref 21:12
    (prints verse 21:12)

$ quran-ref 12:3 12:8 
    (prints verses in range [12:3, 12:4, 12:5, 12:6, 12:7, 12:8])

WIP:
//$ quran-ref -a 3:23-28
//    (prints verses in range [3:23, 3:24, 3:25, 3:26, 3:27, 3:28] with arabic)
//
//$ quran-ref -a 3:10 3:14
//    (prints verses in range [3:10, 3:11, 3:12, 3:13, 3:14] with arabic)

for linux:
config is stored in ~/.config/quran-ref/
data and translations are stored in ~/.local/share/quran-ref/
```

## Todo

### version 1.0.0

- [x] parse flags

- [x] parse verse number input

- [x] get api data

- [x] display literally any valid info

- [x] deploy to [crates.io](https://crates.io/)

- [ ] clean the output of any tags

- [x] select translations

- [x] make offline mode

- [ ] display in Arabic

- [ ] optimize to run in less than 5ms
