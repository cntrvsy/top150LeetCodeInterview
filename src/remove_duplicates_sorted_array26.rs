//easy number 26
// region:    --- Question

//Easy

//Given an integer array nums sorted in non-decreasing order,
//remove the duplicates in-place such that each unique element appears only once.
//The relative order of the elements should be kept the same.
//Then return the number of unique elements in nums.

//Consider the number of unique elements of nums to be k, to get accepted,
//you need to do the following things:

//Change the array nums such that the first k elements of nums contain
//the unique elements in the order they were present in nums initially.
//The remaining elements of nums are not important as well as the size of nums.
//Return k.

//Custom Judge:

//The judge will test your solution with the following code:

//int[] nums = [...]; // Input array
//int[] expectedNums = [...]; // The expected answer with correct length

//int k = removeDuplicates(nums); // Calls your implementation

//assert k == expectedNums.length;
//for (int i = 0; i < k; i++) {
//    assert nums[i] == expectedNums[i];
//}
//If all assertions pass, then your solution will be accepted.

// endregion: --- Question

pub struct Solution {
    pub nums: Vec<i32>,
}

impl Solution {
    pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
        // k keeps track of the index where the next unique element should be placed
        let mut k = 0;
        //loops the entire array
        for i in 0..nums.len() {
            // if nums[i] is not equal to nums[i - 1], place nums[i] is retained
            if i == 0 || nums[i] != nums[i - 1] {
                //swap nums[i] with nums[k]
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}
