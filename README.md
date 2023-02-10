# QuranRef

quran-ref
Display the verses of the quran in various english translations using references

## Installation

- not available for installation yet as project is yet to reach a usable state

## Usage

`~> quran-ref [OPTIONS] <START_CHAPTER:START_VERSE> <END_CHAPTER:END_VERSE>`

```sh
example commands: 
> quran-ref 21:12
> quran-ref 12:3 12:8
> quran-ref -a 3:23 4:1
```

```
OPTIONS:
    -h, --help          shows this help section
    -e, --edit          configure the program
    -a, --arabic        includes the arabic part

config contains island index
config is stored in ~/.config/quran-ref/"
```

## Todo

### version 1.0.0

- [x] parse flags

- [x] parse verse number input

- [ ] parse the database

- [ ] display literally any valid info

- [ ] select translations

- [ ] use build.rs to parse data

- [ ] edit translation selection

- [ ] display in Arabic

- [ ] optimize to run in less than 5ms

- [ ] deploy to [crates.io](https://crates.io/)
