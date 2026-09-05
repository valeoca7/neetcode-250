impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut result = String::new();

        for s in strs {
            result.push_str(&s.len().to_string());
            result.push('#');
            result.push_str(&s);
        }
        result
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let bytes = s.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            let mut j = i;
            
            while bytes[j] != b'#' {
                j += 1;
            }

            let len: usize = s[i..j].parse().unwrap();

            j +=  1;

            let value = s[j..j + len].to_string();
            result.push(value);

            i = j + len;
        }
    result
    }
}
