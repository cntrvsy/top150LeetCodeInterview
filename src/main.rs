// region:    --- Modules

mod error;
mod majority_element169;
pub use error::{Error, Result};
// endregion: --- Modules

fn main() {
    let nums1 = vec![3, 2, 3];
    let result1 = majority_element169::Solution::majority_element(nums1);
    assert_eq!(result1, 3);
    println!("The majority element is: {}", result1);

    let nums2 = vec![2, 2, 1, 1, 1, 2, 2];
    let result2 = majority_element169::Solution::majority_element(nums2);
    assert_eq!(result2, 2);
    println!("The majority element is: {}", result2);
}
