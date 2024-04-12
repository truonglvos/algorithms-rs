// link: https://leetcode.com/problems/invert-binary-tree

use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// impl fmt::Debug for TreeNode {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref n) = root {
            let left = n.borrow().left.clone();
            let right = n.borrow().right.clone();

            let mut n = n.borrow_mut();
            n.left = Self::invert_tree(right);
            n.right = Self::invert_tree(left);
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_tree01() {
        let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let root01 = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: left.clone(),
            right: right.clone(),
        })));

        let root02 = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: right,
            right: left,
        })));

        assert_eq!(Solution::invert_tree(root01), root02)
    }
}
