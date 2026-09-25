impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut rtrv = false;

        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut left = 0;
        let mut right = rows * cols;

        while left < right {
            let mid = left + (right - left) / 2;
            let row = mid / cols;
            let col = mid % cols;
            let value = matrix[row][col];
            if value == target {
                rtrv = true;
                return rtrv;
            } else if value < target {
                left = mid + 1; 
            } else {
                right = mid;
            }
        }
        return rtrv;
    }
}
