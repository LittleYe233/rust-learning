use num::complex::Complex;
use std::ops::Add;
use std::time::Duration;

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    let floats = add(1.2, 3.4);
    let ints = add(10, 20);
    let durations = add(
        Duration::new(5, 0),
        Duration::new(10, 5)
    );
    let complexs = add(
        Complex::new(1, 2),
        Complex { re: 3, im: 4 }
    );

    println!("{}\n{}\n{:?}\n{}", floats, ints, durations, complexs);
}
