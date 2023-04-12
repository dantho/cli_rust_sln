use crate::EntryType::*;
use clap::{App, Arg};
use regex::Regex;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findrs")
        .version("0.1.0")
        .author("Dan Thornton")
        .about("Rust find -- from Command Line Rust by Ken Youens-Clark")
        .arg(
            Arg::with_name("files")
                .value_name("File")
                .help("Filenames or directory names to search")
                .multiple(true)
                .default_value(".")
        )
        .arg(
            Arg::with_name("names")
                .short("n")
                .long("names")
                .help("One or more patterns to find (separated by -o, for 'OR'")
                .multiple(true)
                .required(false)
                .case_insensitive(true)
        )
        .arg(
            Arg::with_name("entry_type")
                .short("t")
                .long("type")
                .help("Type to search for (default is both)")
                .takes_value(true)
                .multiple(true)
                .required(false)
                .case_insensitive(true)
        )
        .get_matches();

        let paths = matches.values_of_lossy("files").unwrap(); // Has a default

        let names = matches
            .value_of("names")
            .map(parse_regex)
            .transpose()
            .map_err(|e| format!("illegal pattern -- {}" , e))?;
        let names = names.into_iter().collect();
        
        let entry_types = matches
            .value_of("entry_type")
            .map(parse_entry_type)
            .transpose()
            .map_err(|e| format!("illegal entry_type -- {}" , e))?;
        let entry_types = entry_types.into_iter().collect();

    Ok(Config {
        paths,
        names,
        entry_types
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

fn parse_regex(s: &str) -> MyResult<Regex> {
    Ok(Regex::new("")?)
}

fn parse_entry_type(s: &str) -> MyResult<EntryType> {
    s.try_into()
}

impl TryInto<EntryType> for &str {
    type Error = Box<dyn Error>;
    fn try_into(self) -> MyResult<EntryType> {
    match &self.to_lowercase()[..] {
        "file" => Ok(File),
        "dir" => Ok(Dir),
        _ => Err(self.into())
    }
    }
}