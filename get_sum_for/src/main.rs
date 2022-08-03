fn main() {
    let left = 1;
    let right = 100;
    let mut sum_odd = 0;
    let mut sum_even = 0;

    if left > right {
        println!("Invalid left bound {} and right bound {}", left, right);
        return;
    }

    for i in left..=right {
        if i % 2 == 0 {
            sum_even += i;
        } else {
            sum_odd += i;
        }
    }

    println!("Range              : {} -> {}", left, right);
    println!("Sum of odd numbers : {}", sum_odd);
    println!("Sum of even numbers: {}", sum_even);
    println!("Sum of all numbers : {}", sum_even + sum_odd);
}
