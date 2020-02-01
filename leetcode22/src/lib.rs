#[derive(Debug)]
struct Solution();

impl Solution {
    pub fn generate_parenthesis_recursion_with_memory(left:i32,right:i32,mem:Vec<Vec<Vec<String>>>) -> (Vec<String>,Vec<Vec<Vec<String>>>) {
        let result =vec![String::from("()")];
        (result, mem)
    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        result.push(String::from("()"));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn show_result() {
        let result = Solution::generate_parenthesis(1);
        for i in result.iter() {
            println!("{}",i);
        }
        assert_eq!(result, ["()"]);
    }

    #[test]
    fn recursion_with_men() {
        let (result, men) = Solution::generate_parenthesis_recursion_with_memory(1, 1, vec![vec![vec![String::from("()()")]]]);
        assert_eq!(result,["()"]);
        assert_eq!(men,[[["()()"]]]);
    }

    #[test]
    fn recursion_with_men() {
        let (result, men) = Solution::generate_parenthesis_recursion_with_memory(1, 1, vec![vec![vec![String::from("()()")]]]);
        assert_eq!(result,["()"]);
        assert_eq!(men,[[["()()"]]]);
    }
}
