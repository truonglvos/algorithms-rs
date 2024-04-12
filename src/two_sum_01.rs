// link: https://leetcode.com/problems/two-sum

use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            let diff = target - nums[i];
            if map.contains_key(&diff) {
                return vec![map[&diff], i as i32];
            }
            map.insert(nums[i], i as i32);
        }

        Vec::new()
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_two_01() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_two_02() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
}
