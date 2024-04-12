// link: https://leetcode.com/problems/merge-two-sorted-lists

use crate::Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (l, None) | (None, l) => l,
            (Some(list1), Some(list2)) => {
                if list1.val < list2.val {
                    Some(Box::new(ListNode {
                        val: list1.val,
                        next: Self::merge_two_lists(list1.next, Some(list2)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: list2.val,
                        next: Self::merge_two_lists(Some(list1), list2.next),
                    }))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_lists01() {
        assert_eq!(
            Solution::merge_two_lists(
                Some(Box::new(ListNode::new(1))),
                Some(Box::new(ListNode::new(2)))
            ),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode::new(2)))
            }))
        );
    }
}
