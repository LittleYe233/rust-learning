use super::insertion;

// See: https://www.geeksforgeeks.org/timsort/
const RUN: usize = 32;

/**
    Split a `[T]`-type container to `[l..m]` and `[m..r]`, and merge them into a
    sorted container in-place.
 */
fn merge<T: Ord + Copy>(arr: &mut [T], m: usize) {
    let n = arr.len();
    let left = arr[..m].to_vec();
    let right = arr[m..].to_vec();

    let mut i = 0usize;
    let mut j = 0usize;
    let mut k = 1usize;

    while i < m && j < n - m {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    if i < m {
        arr[k..].copy_from_slice(&left[i..]);
    }
    if j < n - m {
        arr[k..].copy_from_slice(&right[j..]);
    }
}

pub fn sort<T: Ord + Copy>(arr: &mut [T]) -> &mut [T] {
    let n = arr.len();

    for i in (0..n).step_by(RUN) {
        insertion::sort(&mut arr[i..(n.min(i + RUN))]);
    }

    let mut size = RUN;
    loop {
        if size >= n {
            break;
        }
        size *= 2;

        for left in (0..n).step_by(2 * size) {
            let mid = left + size;
            let right = n.min(left + 2 * size);
            if mid < right {
                merge(&mut arr[left..right], size);
            }
        }
    }

    arr
}
