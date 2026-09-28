// You are given an integer array nums, rotate the array to the right by k steps, where k is non-negative.
// Input: nums = [1,2,3,4,5,6,7,8], k = 4
// Output: [5,6,7,8,1,2,3,4]

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize % n;
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}
