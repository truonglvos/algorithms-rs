// https://leetcode.com/problems/maximum-subarray

use crate::Solution;
use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum: i32 = 0;
        let mut ret: i32 = i32::MIN;
        for &num in nums.iter() {
            sum = max(sum + num, num);
            ret = max(ret, sum);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_sub_array_01() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        )
    }

    #[test]
    fn max_sub_array_02() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1)
    }

    #[test]
    fn max_sub_array_03() {
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23)
    }
}
