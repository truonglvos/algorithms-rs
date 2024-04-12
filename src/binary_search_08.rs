// link: https://leetcode.com/problems/binary-search

use crate::Solution;

impl Solution {
    pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] < target {
                left = mid + 1;
            } else if nums[mid as usize] > target {
                right = mid - 1;
                dbg!(left, right, '1');
            } else {
                return dbg!(mid) as i32;
            }
        }
        -1
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_binary_search01() {
        assert_eq!(Solution::binary_search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn test_binary_search02() {
        assert_eq!(Solution::binary_search(vec![3, 2, 4], 6), -1);
    }

    #[test]
    fn test_binary_search03() {
        assert_eq!(Solution::binary_search(vec![5], -5), -1);
    }

    #[test]
    fn test_binary_search04() {
        assert_eq!(Solution::binary_search(vec![6], 6), 0);
    }
}
