impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = String::new();

        for i in 0..strs[0].len() {

            let temp_char = strs[0].as_bytes()[i];

            for j in 1..strs.len() {
                let bytes = strs[j].as_bytes();

                if i >= bytes.len() || bytes[i] != temp_char {
                    return prefix;
                }

            }

            prefix.push(temp_char as char);

        }

        return prefix
    }
}


