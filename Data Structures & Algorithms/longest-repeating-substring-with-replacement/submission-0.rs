impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();

        let mut freq = [0; 26];
        let mut left = 0;
        let mut max_freq = 0;
        let mut max_len = 0;

        for right in 0..bytes.len() {
            let index = (bytes[right] - b'A') as usize;
            freq[index] += 1;
            max_freq = max_freq.max(freq[index]);

            while (right - left + 1) - max_freq > k as usize {
                let left_index = (bytes[left] - b'A') as usize;
                freq[left_index] -= 1;
                left += 1;
            }
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32

    }
}
