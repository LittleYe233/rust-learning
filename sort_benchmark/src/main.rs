use std::time::Instant;

mod sortbm;

fn main() {
    let vec = vec![5, 6, 3, 7, 1, 10, -5, 114514, -1919810];

    {
        let mut new_vec = vec.to_vec();
        let now = Instant::now();
        sortbm::algo::bubble::sort(&mut new_vec);
        let dur = now.elapsed().as_nanos();
        println!(
            "Sorted array (bubble sort):       {:?} elapsed {} ns",
            new_vec, dur
        );
    }

    {
        let mut new_vec = vec.to_vec();
        let now = Instant::now();
        sortbm::algo::insertion::sort(&mut new_vec);
        let dur = now.elapsed().as_nanos();
        println!(
            "Sorted array (insertion sort):    {:?} elapsed {} ns",
            new_vec, dur
        );
    }

    {
        let mut new_vec = vec.to_vec();
        let now = Instant::now();
        sortbm::algo::selection::sort(&mut new_vec);
        let dur = now.elapsed().as_nanos();
        println!(
            "Sorted array (selection sort):    {:?} elapsed {} ns",
            new_vec, dur
        );
    }

    {
        let mut new_vec = vec.to_vec();
        let now = Instant::now();
        sortbm::algo::tim::sort(&mut new_vec);
        let dur = now.elapsed().as_nanos();
        println!(
            "Sorted array (timsort):           {:?} elapsed {} ns",
            new_vec, dur
        );
    }

    println!(
        "Original array:                   {:?}",
        vec
    );
}
