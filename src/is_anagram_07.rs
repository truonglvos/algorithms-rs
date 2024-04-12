// link: https://leetcode.com/problems/valid-anagram

use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map: HashMap<char, i32> = HashMap::new();
        for temp_s in s.chars() {
            if map.contains_key(&temp_s) {
                map.insert(temp_s, map.get(&temp_s).unwrap() + 1);
            } else {
                map.insert(temp_s, 1);
            }
        }

        for temp_t in t.chars() {
            if !map.contains_key(&temp_t) {
                return false;
            } else {
                let remain = map.get(&temp_t).unwrap() - 1;
                if remain == 0 {
                    map.remove(&temp_t);
                } else {
                    map.insert(temp_t, remain);
                }
            }
        }

        map.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram01() {
        assert_eq!(
            Solution::is_anagram(String::from("rat"), String::from("car")),
            false
        );
    }

    #[test]
    fn test_is_anagram02() {
        assert_eq!(
            Solution::is_anagram(String::from("anagram"), String::from("nagaram")),
            true
        )
    }
}
