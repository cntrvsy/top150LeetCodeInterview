// region:    --- Modules

mod error;

mod merge_sorted_arrays88;

pub use error::{Error, Result};

// endregion: --- Modules

fn main() -> Result<()> {
    // example one
    let mut example1 = merge_sorted_arrays88::Solution {
        nums1: vec![1, 2, 3, 0, 0, 0],
        nums2: vec![2, 5, 6],
        m: 3,
        n: 3,
    };

    merge_sorted_arrays88::Solution::merge(
        &mut example1.nums1,
        example1.m,
        &mut example1.nums2,
        example1.n,
    );

    println!("{:?}", example1.nums1);

    // example two
    let mut example2 = merge_sorted_arrays88::Solution {
        nums1: vec![1],
        nums2: vec![],
        m: 1,
        n: 0,
    };

    merge_sorted_arrays88::Solution::merge(
        &mut example2.nums1,
        example2.m,
        &mut example2.nums2,
        example2.n,
    );

    println!("{:?}", example2.nums1);

    Ok(())
}
