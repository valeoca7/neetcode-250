// struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left:i64 = 1;
        let mut right: i64 = *piles.iter().max().unwrap() as i64;
        while left < right {
            let speed = left  + (right -left) / 2;

            let mut hours_needed: i64 = 0;

            for &pile in &piles {
                let pile = pile as i64;
                hours_needed += (pile + speed - 1) / speed;
                if hours_needed > h as i64{
                    break;
                }
            }
            if hours_needed <= h as i64 {
                right =speed;

            } else {
                left = speed +1;
            }
        }
        left as i32
    }
}
