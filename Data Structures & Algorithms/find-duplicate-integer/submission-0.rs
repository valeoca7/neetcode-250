impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let next = |index: i32| -> i32 {
            nums[index as usize]
        };
        
        // phase 1
        let mut slow = nums[0];
        let mut fast = nums[0];

        loop {
            slow = next(slow);
            fast = next(next(fast));
            if slow == fast {
                break;
            }
        }

        // phase 2
        let mut finder = nums[0];
        while finder != slow {
            finder = next(finder);
            slow = next(slow);
        }

        finder
    }
}
