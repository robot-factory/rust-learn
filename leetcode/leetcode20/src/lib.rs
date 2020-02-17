pub struct Solution();
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
        let mut map = HashMap::with_capacity(3);
        map.insert(')', '(');
        map.insert(']', '[');
        map.insert('}', '{');

        for c in s.chars() {
            if ['(', '[', '{'].contains(&c) {
                stack.push(c)
            } else {
                let need = map.get(&c);
                let last = stack.pop();
                if need.unwrap() != &last.unwrap_or_default() {
                    return false;
                }
            }
        }
        stack.is_empty()
    }

    pub fn is_valid2(s: String) -> bool {
        let mut stack = Vec::new();

        for c in s.chars() {
            stack.push(match c {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                _ => {
                    if Some(c) == stack.pop() {
                        continue;
                    } else {
                        return false;
                    }
                }
            })
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn is_valid() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
        assert_eq!(Solution::is_valid(String::from("(]")), false);
        assert_eq!(Solution::is_valid(String::from("([)]")), false);
        assert_eq!(Solution::is_valid(String::from("{[]}")), true);
    }
}
