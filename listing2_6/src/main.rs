use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("a = {}", a); // <re>+<im>i
    println!("b = {}", b);
    println!("result = {} + {}i", result.re, result.im); // <re> + <im>i
}
