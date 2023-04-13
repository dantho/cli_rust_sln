use crate::EntryType::*;
use clap::{App, Arg};
use regex::Regex;
use std::{error::Error, borrow::Cow};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
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
            Arg::with_name("paths")
                .value_name("PATH")
                .help("Limit search to these filenames or directories")
                .multiple(true)
                .default_value(".")
        )
        .arg(
            Arg::with_name("names")
                .value_name("NAME")
                .short("n")
                .long("name")
                .help("Regex patterns to search for (separated by -o, for 'OR')")
                .takes_value(true)
                .multiple(true)
                .required(false)
                .case_insensitive(true)
        )
        .arg(
            Arg::with_name("types")
                .value_name("TYPE")
                .short("t")
                .long("type")
                .help("Type(s) to search for (default is both 'file' and 'dir')")
                .possible_values(&["f", "file", "d", "dir"])
                .takes_value(true)
                .multiple(true)
                .required(false)
                .case_insensitive(true)
        )
        .get_matches();

        let paths = matches.values_of_lossy("paths").unwrap(); // Has a default

        let names = matches
            .values_of_lossy("names")
            .map(|names| {
                names.into_iter()
                    .map(|name| {
                        Regex::new(&name)
                            .map_err(|_| format!("Invalid --name \"{}\"", name))  
                    }).collect::<Result<Vec<_>, _>>()
            }).transpose()?
            .unwrap_or_default();
        
        let entry_types = matches
            .values_of_lossy("types")
            .map(parse_entry_types)
            .transpose()
            .map_err(|e| format!("illegal type -- {}" , e))?;

        let mut entry_types = entry_types.unwrap_or_default();
        entry_types.sort();
        entry_types.dedup();

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

fn parse_entry_types(vs: Vec<String>) -> MyResult<Vec<EntryType>> {
    Ok(vs.iter().map(|s|s[..].try_into()).flatten().collect())
}

impl TryInto<EntryType> for &str {
    type Error = Box<dyn Error>;
    fn try_into(self) -> MyResult<EntryType> {
        match &self.to_lowercase()[..] {
            "file" | "f" => Ok(File),
            "dir" | "d" => Ok(Dir),
            _ => Err(self.into())
        }
    }
}