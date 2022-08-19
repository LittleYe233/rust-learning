use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App, Arg};

fn main() {
    let args = App::new("regexp_args_with_file")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .help("File to search")
            .takes_value(true)
            .required(true))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let input = args.value_of("input").unwrap();

    let re = Regex::new(pattern).unwrap();
    
    let f = File::open(input).unwrap();
    let mut reader = BufReader::new(f);

    let mut quote = String::new();
    reader.read_to_string(&mut quote).unwrap();
    // see: https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust
    for word in quote.split_whitespace() {
        match re.find(&word) {
            Some(_) => println!("{}", word),
            None => (),
        }
    }
}
