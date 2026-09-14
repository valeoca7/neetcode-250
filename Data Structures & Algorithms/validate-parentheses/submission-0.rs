impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<u8> = Vec::new();
        for ch in s.bytes() {
            match ch {
                b'(' | b'[' | b'{' => { stack.push(ch); }
                b')' => { // incorrect closing symbol
                    if stack.pop() != Some(b'(') {
                        return false;
                    }
                }
                b']' => { // incorrect closing symbol
                    if stack.pop() != Some(b'[') {
                        return false;
                    }
                }
                b'}' => { // incorrect closing symbol
                    if stack.pop() != Some(b'{') {
                        return false;
                    }
                }
                _ => { // handle input unexected symbols
                    panic!("Unexpected input char");
                }
            }
        }

        stack.is_empty() // True if no open brackets left
    }
}
