impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {

        let mut nset: HashSet<i32> = nums.into_iter().collect();
        let mut rtrv = 0;
        
        for &num in &nset {
            if nset.contains(&(num-1)) {
                continue;
            }

            let mut current = num;
            let mut count = 0;

            while nset.contains(&current) {
                count += 1;
                current += 1;
            }

            rtrv  = rtrv.max(count);
        }

        rtrv
    }
}
