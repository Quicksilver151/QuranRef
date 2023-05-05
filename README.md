# QuranRef

quran-ref
Display the verses of the quran in various english translations using references

- (currently outputs both translations from sahih international and Dr.Mustafa Khattab, the Clear Quran)

- (Currently only works online)

## Installation

- run `cargo install quran-ref`

## Usage

`$ quran-ref [OPTIONS] <START_CHAPTER:START_VERSE> <END_CHAPTER:END_VERSE>`

or

`$ quran-ref [OPTIONS] <START_CHAPTER:START_VERSE>-<END_VERSE>`

## Options:

```
    -h, --help          shows this help section
    -e, --edit          configure the program
    -a, --arabic        includes the arabic part

config contains island index
config is stored in ~/.config/quran-ref/"
```

## Examples

```sh
$ quran-ref 21:12

$ quran-ref 12:3 12:8 
    (prints verses in range [12:3, 12:4, 12:5, 12:6, 12:7, 12:8])

$ quran-ref -a 3:23-28
    (prints verses in range [3:23, 3:24, 3:25, 3:26, 3:27, 3:28] with arabic)

$ quran-ref -a 3:10 3:14
    (prints verses in range [3:10, 3:11, 3:12, 3:13, 3:14] with arabic)
```

## Todo

### version 1.0.0

- [x] parse flags

- [x] parse verse number input

- [x] get api data

- [x] display literally any valid info

- [ ] select translations

- [ ] make offline mode

- [ ] display in Arabic

- [ ] optimize to run in less than 5ms

- [ ] deploy to [crates.io](https://crates.io/)
