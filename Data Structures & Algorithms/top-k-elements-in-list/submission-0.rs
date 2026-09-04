impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq = HashMap::new();

        for num in &nums {
            *freq.entry(*num).or_insert(0) += 1;
        }

        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];

        for (num, count) in freq {
            buckets[count].push(num);
        }
        
        let mut result = Vec::new();

        for i in (0..buckets.len()).rev() {
            for num in &buckets[i] {
                result.push(*num);

                if result.len() == k as usize {
                    return result;
                }
            }
        }

        result
    }
}
