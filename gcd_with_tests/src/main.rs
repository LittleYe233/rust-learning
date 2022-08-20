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

fn main() {}
