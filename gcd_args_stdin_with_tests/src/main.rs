use clap::{App, Arg};
use std::io;
use std::io::prelude::*;

fn gcd(n: u64, m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    let (mut x, mut y) = (n, m);

    // see: https://en.wikipedia.org/wiki/Euclidean_algorithm
    while y != 0 {
        (x, y) = (y, x % y);
    }

    x
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

fn main() {
    let numbers: Vec<u64>;

    // parse command-line arguments
    let args = App::new("gcd_args_stdin_with_tests")
        .version("0.1")
        .about("calculate the greatest common divisor of two non-negative integers")
        .arg(
            Arg::with_name("numbers")
                .help("The non-negative numbers to calculate the GCD with")
                .value_parser(clap::builder::RangedU64ValueParser::<u64>::new())
                .multiple(true)
                .takes_value(true),
        )
        .get_matches();

    let numbers_arg = args.get_many::<u64>("numbers");

    if numbers_arg.is_none() {
        // No arguments given. Accept values from stdin.
        println!("Please input several non-negative numbers (separate with spaces):");

        let mut s = String::new();
        let stdin = io::stdin();
        let mut reader = stdin.lock();

        while reader.read_line(&mut s).unwrap() == 1 {} // line separator takes up 1 byte.

        numbers = s.trim().split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    } else {
        // Accept values from command-line arguments.
        numbers = numbers_arg.unwrap().map(|x| *x).collect();
    }

    // calculate the result
    let mut result: u64 = numbers[0];

    for x in &numbers[1..] {
        result = gcd(result, *x);
    }

    println!("{}", result);
}
