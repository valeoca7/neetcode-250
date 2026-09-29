impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 - 1;

        while left <= right {
            let mid = left +(right -left) /2;
            let left_i = left as usize;
            let mid_i = mid as usize;
            let right_i = right as usize;

            if nums[mid_i] == target{
                return mid;
            }
            if nums[left_i] <= nums[mid_i]{
                if nums[left_i] <= target && target < nums[mid_i]{
                    right = mid -1;
                }else{
                    left = mid +1;
                }
            }
            else{
                if nums[mid_i] < target && target <= nums[right_i]{
                    left = mid + 1;
                }else{
                    right = mid -1;
                }
            }
        }
        -1
    }
}
