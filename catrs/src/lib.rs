use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type CatResult<T> = Result<T, Box<dyn Error>>;
pub fn run(arg_data: Config) -> CatResult<()> {
    for file in &arg_data.files {
        let reader = open(file);
        match reader {
            Err(e) => {
                eprintln!("Failed to open {}: {}", file, e);
            },
            _ => {
                let n = arg_data.number_lines;
                let b = arg_data.number_nonblank_lines;
                let mut ln_num = 0;
                for line in reader?.lines() {
                    let text = line?;
                    let ln_txt = if n || (b && text.trim().is_empty()) {
                        ln_num+=1;
                        format!("{:>6}     ", ln_num)
                    } else {
                        "".to_string()
                    };
                    println!("{}{}", ln_txt, text);
                }
            },
        }
    }

    Ok(())
}

pub fn get_args() -> CatResult<Config> {
    let matches = App::new("catrs")
        .version("0.1.0")
        .author("Dan Thornton <dan.thornton@thornton.me")
        .about("Cat in Rust")
        .arg(
            Arg::with_name("file_or_stdin")
                .value_name("File")
                .help("Input filename or '-' for STDIN")
                .required(false)
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number_lines")
                .help("Add line numbers to EVERY line.")
                .takes_value(false)
                .conflicts_with("number-nonblank")
        )
        .arg(
            Arg::with_name("number-nonblank")
                .short("b")
                .long("number_nonblank_lines")
                .help("Add line numbers only to lines that are not blank.")
                .takes_value(false)
        )
        .get_matches();

        let files = matches.values_of_lossy("file_or_stdin").unwrap();
        let number_nonblank_lines = matches.is_present("number-nonblank");
        let number_lines = matches.is_present("number");
        Ok(Config {
            files,
            number_lines,
            number_nonblank_lines,
        })

    }

    pub fn open(filename: &str) -> CatResult<Box<dyn BufRead>> {
        match filename {
            "-" => Ok(Box::new(BufReader::new(io::stdin()))),
            _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
        }
    }