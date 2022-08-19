use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("README.md").unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    loop {
        if reader.read_line(&mut line).unwrap() == 0 {
            break;
        }

        line = line.trim_end_matches('\n').to_string();

        println!("{} ({} bytes long)", line, line.len());

        line.truncate(0);
    }
}
