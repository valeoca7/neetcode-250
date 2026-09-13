impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut charts = HashSet::new();
        let mut left = 0;

        let mut longest = 0;

        for right in 0..bytes.len() {
            while charts.contains(&bytes[right]) { 
                charts.remove(&bytes[left]);
                left += 1;
            }
            charts.insert(bytes[right]);
            longest = longest.max(right-left+1)
        }
        longest as i32
    }
}
