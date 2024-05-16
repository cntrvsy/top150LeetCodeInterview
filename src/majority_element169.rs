//easy number 169
// region:    --- Question

//Easy

//Given an array nums of size n, return the majority element.

//The majority element is the element that appears more than ⌊n / 2⌋ times.
//You may assume that the majority element always exists in the array.

//Example 1:

//Input: nums = [3,2,3]
//Output: 3
//Example 2:

//Input: nums = [2,2,1,1,1,2,2]
//Output: 2
// endregion: --- Question

pub struct Solution {
    pub nums: Vec<i32>,
}

impl Solution {
    //Boyer-Moore Voting Algorithm,
    //which finds the majority element in linear time and O(1) space.
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // count keeps track of the count of the current candidate.
        // candidate keeps track of the current candidate.
        let mut count = 0;
        let mut candidate = 0;

        //loop through the array
        for &num in &nums {
            // if count is 0, set candidate to num
            // this ensures that candidate will be the element that appears more than n/2 times
            if count == 0 {
                candidate = num;
            }
            // if num is equal to candidate, increment count
            if num == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }
        candidate
    }
}

// region:    --- Main

// fn main() {
//     let nums1 = vec![3, 2, 3];
//     let result1 = majority_element169::Solution::majority_element(nums1);
//     assert_eq!(result1, 3);
//     println!("The majority element is: {}", result1);

//     let nums2 = vec![2, 2, 1, 1, 1, 2, 2];
//     let result2 = majority_element169::Solution::majority_element(nums2);
//     assert_eq!(result2, 2);
//     println!("The majority element is: {}", result2);
// }

// endregion: --- Main
