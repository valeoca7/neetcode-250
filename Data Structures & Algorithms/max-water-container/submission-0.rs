impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();

        let mut left = 0;
        let mut right = n - 1;
        let mut vol = 0;
        // let mut first_i = 0;
        // let mut second_i = 0;

        while !(left == right) {
            let this_vol = (right - left) * min(heights[left] as usize, heights[right] as usize);

            if this_vol > vol {
                vol = this_vol;
            }

            if heights[left] > heights[right] {
                right -= 1;
            } else {
                left += 1;
            }

        }

        return vol as i32
        
    }
}
