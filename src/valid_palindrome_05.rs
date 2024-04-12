// link: https://leetcode.com/problems/valid-palindrome

use crate::Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<_> = s
            .chars()
            .filter_map(|c| c.is_ascii_alphanumeric().then(|| c.to_ascii_lowercase()))
            .collect();
        s.iter().eq(s.iter().rev())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome01() {
        assert_eq!(
            Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")),
            true
        );
    }

    #[test]
    fn test_is_palindrome02() {
        assert_eq!(Solution::is_palindrome(String::from("race a car")), false);
    }
}
