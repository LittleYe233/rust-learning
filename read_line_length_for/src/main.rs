use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("README.md").unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let mut line = line_.unwrap();
        line = line.trim_end_matches('\n').to_string();

        println!("{} ({} bytes long)", line, line.len());
    }
}
