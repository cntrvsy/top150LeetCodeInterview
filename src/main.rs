// region:    --- Modules

mod error;
mod remove_element27;
pub use error::{Error, Result};
// endregion: --- Modules

fn main() -> Result<()> {
    let mut example = remove_element27::Solution {
        nums: vec![0, 1, 2, 2, 3, 0, 4, 2],
        val: 2,
    };

    let ans = remove_element27::Solution::remove_element(&mut example.nums, example.val);

    println!("{:?}", ans);

    println!("{:?}", example.nums);

    Ok(())
}
