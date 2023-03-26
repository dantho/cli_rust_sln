use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
}

type CrateResult<T> = Result<T, Box<dyn Error>>;

pub fn run(arg_data: Config) -> CrateResult<()> {
    for filepath in &arg_data.files {
        // Create a file inside of `std::env::temp_dir()`.
        let mut tmpfile = tempfile::NamedTempFile::new()?;
        {
            let reader = open(filepath);
            match reader {
                Err(e) => {
                    eprintln!("Failed to open {}: {}", filepath, e);
                },
                _ => {
                    // This is a no-brainer for all but the LAST line
                    // If the last line is unterminated, we shouldn't terminate it
                    let ends_in_linefeed = true;
                    let mut lines = reader?.lines();
                    let mut last_line = lines.next().unwrap();
                    for line in lines {
                        writeln!(tmpfile,"{}", last_line?)?;
                        last_line = line;
                    }
                    if ends_in_linefeed {
                        writeln!(tmpfile,"{}", last_line?)?;
                    } else {
                        write!(tmpfile,"{}", last_line?)?;
                    }
                },
            }
        } // drop reader
        // Now replace original file with tmpfile
        tmpfile.persist(filepath)?;
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