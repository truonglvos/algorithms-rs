// link: https://leetcode.com/problems/valid-parentheses

use crate::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | '}' | ']' => {
                    if Some(c) != stack.pop() {
                        return false;
                    }
                }
                _ => (),
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid01() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
    }

    #[test]
    fn test_is_valid02() {
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    }

    #[test]
    fn test_is_valid03() {
        assert_eq!(Solution::is_valid(String::from("(]")), false);
    }
}
