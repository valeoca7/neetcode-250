impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count = [0; 26];

        for c in s.bytes() {
            count[(c - b'a') as usize] += 1;
        }
        for c in t.bytes() {
            count[(c - b'a') as usize] -= 1;
        }
        
        return count.iter().all(|&x| x == 0)
    }
}
