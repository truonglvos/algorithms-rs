use std::{cell::RefCell, rc::Rc};

mod balanced_binary_tree_12;
mod best_time_to_buy_and_sell_stock_04;
mod binary_search_08;
mod flood_fill_09;
mod invert_binary_tree_06;
mod is_anagram_07;
mod lowest_common_ancestor_of_a_binary_search_tree_11;
mod maximum_subarray_10;
mod merge_two_sorted_list_03;
mod two_sum_01;
mod valid_palindrome_05;
mod valid_parentheses_02;

pub mod common;

fn main() {
    println!("Hello, world!");
}

pub struct Solution {}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        let (mut i, mut best) = (0, 0);
        while i < nums.len() {
            if nums[i] % 2 == 1 || nums[i] > threshold {
                i += 1;
                continue;
            }
            let mut j = i + 1;
            while j < nums.len() {
                if nums[j] > threshold || nums[j] % 2 == nums[j - 1] % 2 {
                    break;
                }
                j += 1;
            }
            best = best.max(j - i);
            i = j;
        }
        best as i32
    }

    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut r: Vec<i32> = Vec::new();
        let mut temp = 0;
        for num in nums1.iter() {
            if nums2.contains(num) {
                temp += 1;
            }
        }
        r.push(temp);
        temp = 0;
        for num in nums2.iter() {
            if nums1.contains(num) {
                temp += 1;
            }
        }
        r.push(temp);
        r
    }

    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut r: Vec<i32> = Vec::new();
        for num in nums.iter() {
            let temp = *num;
            if temp < 10 {
                r.push(temp);
            } else {
                r = [
                    r,
                    Vec::from_iter(
                        temp.to_string()
                            .chars()
                            .into_iter()
                            .map(|c| c.to_string().parse::<i32>().unwrap()),
                    ),
                ]
                .concat();
            }
        }
        r
    }
    //
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_longest_alternating_subarray01() {
        assert_eq!(
            Solution::longest_alternating_subarray(vec![2, 3, 4, 5], 4),
            3
        );
    }

    #[test]
    fn test_longest_alternating_subarray02() {
        assert_eq!(Solution::longest_alternating_subarray(vec![1, 2], 2), 1);
    }

    #[test]
    fn test_longest_alternating_subarray03() {
        assert_eq!(
            Solution::longest_alternating_subarray(vec![3, 2, 5, 4], 5),
            3
        );
    }

    #[test]
    fn find_intersection_values01() {
        assert_eq!(
            Solution::find_intersection_values(vec![4, 3, 2, 3, 1], vec![2, 2, 5, 2, 3, 6]),
            vec![3, 4]
        );
    }

    #[test]
    fn find_intersection_values02() {
        assert_eq!(
            Solution::find_intersection_values(vec![3, 4, 2, 3], vec![1, 5]),
            vec![0, 0]
        );
    }

    #[test]
    fn separate_digits01() {
        assert_eq!(
            Solution::separate_digits(vec![13, 25, 83, 77]),
            vec![1, 3, 2, 5, 8, 3, 7, 7]
        );
    }

    #[test]
    fn separate_digits02() {
        assert_eq!(
            Solution::separate_digits(vec![7, 1, 3, 9]),
            vec![7, 1, 3, 9]
        );
    }

    //
}
