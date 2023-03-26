use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
}

type CrateResult<T> = Result<T, Box<dyn Error>>;

pub fn run(arg_data: Config) -> CrateResult<()> {
    for file in &arg_data.files {
        let reader = open(file);
        match reader {
            Err(e) => {
                eprintln!("Failed to open {}: {}", file, e);
            },
            _ => {
                for line in reader?.lines() {
                    println!("{}", line?);
                }
            },
        }
    }

    Ok(())
}

pub fn get_args() -> CrateResult<Config> {
    let matches = App::new("catrs")
        .version("0.1.0")
        .author("Dan Thornton <dan.thornton@thornton.me")
        .about("Convert line endings in-place on files or stdin/stdout")
        .arg(
            Arg::with_name("file_list")
                .value_name("File")
                .help("Input filename or leave blank for STDIN")
                .required(false)
                .multiple(true)
                .default_value("-"),
        )
        .get_matches();

        let files = matches.values_of_lossy("file_list").unwrap();
        Ok(Config {
            files,
        })

    }

    pub fn open(filename: &str) -> CrateResult<Box<dyn BufRead>> {
        match filename {
            "-" => Ok(Box::new(BufReader::new(io::stdin()))),
            _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
        }
    }