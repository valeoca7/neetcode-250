impl Solution {
    // You are given two strings s1 and s2.
    // Return true if s2 contains a permutation of s1, or false otherwise. That means   if a permutation of s1 exists as a substring of s2, then return true.
    // Both strings only contain lowercase letters.
    
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        let mut target = [0; 26];
        let mut window = [0; 26];

        // process first step of comparison
        for i in 0..s1.len() {
            target[(s1[i] - b'a') as usize] += 1;
            window[(s2[i] - b'a') as usize] += 1;
        }

        if target == window {
            return true;
        }

        // process other steps till the end of s2
        for right in s1.len()..s2.len() {
            // add new char to the tmp frequency map
            window[(s2[right] - b'a') as usize] += 1;
            // remove left char from previous iter step
            window[(s2[right - s1.len()] - b'a') as usize] -= 1;

            if target == window {
                return true;
            }
        }

        // no occurency was found
        false

    }
}
