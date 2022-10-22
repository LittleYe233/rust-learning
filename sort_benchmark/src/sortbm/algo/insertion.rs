use std::cmp::Ord;
use std::marker::Copy;

// See: https://www.geeksforgeeks.org/timsort/
pub fn sort<T: Ord + Copy>(arr: &mut [T]) -> &mut [T] {
    let mut t: T;
    let mut j: usize;
    let mut reached_first: bool;
    let n = arr.len();
    
    for i in 1..n {
        j = i - 1;
        t = arr[i];
        reached_first = false;
        // You can move the element and the cursor simultaneously.
        while arr[j] > t {
            arr[j + 1] = arr[j];
            if j > 0 {
                j -= 1;
            } else {
                reached_first = true;
                break;
            }
        }
        if reached_first {
            arr[0] = t;
        } else {
            arr[j + 1] = t;
        }
    }

    arr
}
