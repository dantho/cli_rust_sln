use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

type HeadResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> HeadResult<()> {
    // println!("{:#?}", config);
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_) => println!("Successfully opened {}", filename),
        }
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> HeadResult<usize> {
    match val.parse() {
        Ok(num) => Ok(num),
        Err(_) => Err(val.into())
    }
}

pub fn get_args() -> HeadResult<Config> {
    let matches = App::new("Headrs")
        .version("0.1.0")
        .author("Dan Thornton <dan.thornton@thornton.me")
        .about("Head in Rust")
        .arg(
            Arg::with_name("file_or_stdin")
                .value_name("File")
                .help("Input filename or '-' for STDIN")
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .help("Number of lines")
                .case_insensitive(true)
                .default_value("10")
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("Number of bytes")
                .takes_value(true)
                .required(false)
                .case_insensitive(true)
                .conflicts_with("lines")
        )
        .get_matches();

        let files = matches.values_of_lossy("file_or_stdin").unwrap(); // Has a default

        let lines = matches
            .value_of("lines")
            .map(parse_positive_int)
            .transpose()
            .map_err(|e| format!("illegal line count -- {}" , e))?.unwrap();
        let bytes = matches
            .value_of("bytes")
            .map(parse_positive_int)
            .transpose()
            .map_err(|e| format!("illegal byte count -- {}" , e))?;

        Ok(Config {
            files,
            lines,
            bytes,
        })

    }

    pub fn open(filename: &str) -> HeadResult<Box<dyn BufRead>> {
        match filename {
            "-" => Ok(Box::new(BufReader::new(io::stdin()))),
            _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
        }
    }


    #[test]
    fn test_parse_positive_int() {
        // 3 is an OK positive integer
        let res = parse_positive_int("3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 3);

        // -3 is a BAD positive integer
        let res = parse_positive_int("-3");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "-3".to_string());

        // non-numeric string is an error
        let res = parse_positive_int("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo".to_string());
    }