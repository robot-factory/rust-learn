#[derive(Debug)]
struct Solution();

impl Solution {
    pub fn dfs(left: i32, right: i32, pre: String, result: &mut Vec<String>) {
        if left == 0 && right == 0 {
            result.push(pre.clone());
        }
        if left > 0 {
            Self::dfs(left-1, right, format!("{}(", pre.clone()), result);
        }
        if right > left {
            Self::dfs(left, right-1, format!("{})",pre.clone()),result);
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result:Vec<String> = vec![];
        Self::dfs(n, n, String::from(""), &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn show_result() {
        let result = Solution::generate_parenthesis(4);
        for i in result.iter() {
            println!("{}", i);
        }
        assert_eq!(
            result,
            [
                "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
                "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()"
            ]
        );
    }

    // #[test]
    // fn recursion_with_men() {
    //     let (result, men) = Solution::generate_parenthesis_recursion_with_memory(1, 1, vec![vec![vec![String::from("()()")]]]);
    //     assert_eq!(result,["()"]);
    //     assert_eq!(men,[[["()()"]]]);
    // }

    // #[test]
    // fn recursion_with_men2() {
    //     let (result, men) = Solution::generate_parenthesis_recursion_with_memory(1, 1, vec![vec![vec![String::from("()()")]]]);
    //     assert_eq!(result,["()"]);
    //     assert_eq!(men,[[["()()"]]]);
    // }
}
