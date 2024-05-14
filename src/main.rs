// region:    --- Modules

mod error;
mod remove_duplicates_sorted_array26;
pub use error::{Error, Result};
// endregion: --- Modules

fn main() -> Result<()> {
    let mut example = remove_duplicates_sorted_array26::Solution {
        nums: vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
    };

    let ans = remove_duplicates_sorted_array26::Solution::remove_duplicates(&mut example.nums);

    println!("{:?}", ans);

    // truncate to the first "ans" elements
    example.nums.truncate(ans as usize);

    println!("{:?}", example.nums);

    Ok(())
}
