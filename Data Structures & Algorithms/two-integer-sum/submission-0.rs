impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();

        for i in 0..nums.len() {
            let diff = target - nums[i];

            if let Some(&j) = seen.get(&diff) {
                return vec![j as i32, i as i32];
            }
            seen.insert(nums[i], i);
        }

        vec![]
    }
}
