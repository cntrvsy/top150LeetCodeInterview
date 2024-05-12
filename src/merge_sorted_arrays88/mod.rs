pub struct Solution {
    pub nums1: Vec<i32>,
    pub nums2: Vec<i32>,
    pub m: i32,
    pub n: i32,
}

impl Solution {
    pub fn merge(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
        // let get the actual indices
        let mut i = m - 1; // m - 1 is the index of the last element in nums1
        let mut j = n - 1; // n - 1 is the index of the last element in nums2
        let mut k = m + n - 1; // m + n - 1 is the index of the last element in the merged array

        while i >= 0 && j >= 0 {
            if nums1[i as usize] > nums2[j as usize] {
                // if nums1[i] > nums2[j], place nums1[i] at the end of the merged array
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            } else {
                // if nums1[i] <= nums2[j], place nums2[j] at the end of the merged array
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            }
            k -= 1;
        }

        while j >= 0 {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
            k -= 1;
        }
    }
}

// You are given two integer arrays nums1 and nums2, sorted in non-decreasing order,
//and two integers m and n, representing the number of elements in nums1 and nums2 respectively.

// Merge nums1 and nums2 into a single array sorted in non-decreasing order.

// The final sorted array should not be returned by the function, but instead be stored
//inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements
//denote the elements that should be merged, and the last n elements are set to 0 and should be ignored.
//nums2 has a length of n.

// Example 1:

// Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
// Output: [1,2,2,3,5,6]
// Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
// The result of the merge is [1,2,2,3,5,6] with the underlined elements coming from nums1.

// Example 2:

// Input: nums1 = [1], m = 1, nums2 = [], n = 0
// Output: [1]
// Explanation: The arrays we are merging are [1] and [].
// The result of the merge is [1].
