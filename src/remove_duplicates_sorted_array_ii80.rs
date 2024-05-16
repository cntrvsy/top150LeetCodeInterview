//medium number 80
// region:    --- Question

//Medium

//Given an integer array nums sorted in non-decreasing order,
//remove some duplicates in-place such that each unique element appears at most twice.
//The relative order of the elements should be kept the same.

//Since it is impossible to change the length of the array in some languages,
//you must instead have the result be placed in the first part of the array nums.
//More formally, if there are k elements after removing the duplicates,
//then the first k elements of nums should hold the final result.
//It does not matter what you leave beyond the first k elements.

//Return k after placing the final result in the first k slots of nums.

//Do not allocate extra space for another array.
//You must do this by modifying the input array in-place with O(1) extra memory.

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
        // If the array is empty or has only one element, no need to process further
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        // k keeps track of the index where the next unique element should be placed
        let mut k = 2;

        //loops the entire array
        for i in 2..nums.len() {
            // If the current element `nums[i]` is not equal to the element two places
            //before it `nums[k - 2]`
            // it means this element can be included in the result array.
            if nums[i] != nums[k - 2] {
                //swap nums[i] with nums[k]
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}
