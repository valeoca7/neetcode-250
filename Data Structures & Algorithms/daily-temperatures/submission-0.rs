impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();

        let mut result = vec![0; n];
        let mut st: Vec<usize> = Vec::new();

        for i in 0..n {
            while let Some(&prev_i) = st.last() {
                if temperatures[i] <= temperatures[prev_i] {
                    break;
                }

                st.pop();
                result[prev_i] = (i - prev_i) as i32;
            }
            st.push(i);
        }
        result
    }
}
