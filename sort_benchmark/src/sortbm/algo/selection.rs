use std::cmp::Ord;

// See: https://en.wikipedia.org/wiki/Selection_sort#Implementations
pub fn sort<T: Ord>(arr: &mut [T]) -> &mut [T] {
    let n = arr.len();
    let mut j_min: usize;

    for i in 0..n - 1 {
        j_min = i;
        for j in i + 1..n {
            if arr[j] < arr[j_min] {
                j_min = j;
            }
        }
        if j_min != i {
            arr.swap(i, j_min);
        }
    }

    arr
}
