impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut freq = [0u8; 26];

            for b in s.bytes() {
                let index = (b - b'a') as usize;
                freq[index] += 1;
            }

            groups
                .entry(freq)
                .or_insert(Vec::new())
                .push(s);
        }

        groups.into_values().collect()
    }
}
