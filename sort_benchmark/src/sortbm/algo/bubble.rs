use std::cmp::Ord;

// See: https://en.wikipedia.org/wiki/Bubble_sort#Optimizing_bubble_sort
pub fn sort<T: Ord>(arr: &mut [T]) -> &mut [T] {
    let mut n = arr.len();
    let mut newn: usize;
    
    loop {
        newn = 0;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                newn = i;
            }
        }
        n = newn;

        if n <= 1 {
            break;
        }
    }

    arr
}
