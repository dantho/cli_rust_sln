use clap::{App, Arg};

fn main() {
    println!("std out");
    let matches = App::new("echors")
        .version("0.1.0")
        .author("Dan Thornton <dan.thornton@thornton.me")
        .about("Echo in Rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    println!("{:#?}", matches);
}