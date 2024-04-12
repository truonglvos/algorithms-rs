// link: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/
// Definition for a binary tree node.
use crate::Solution;
use std::cell::RefCell;
use std::cmp::Ordering::{Greater, Less};
use std::rc::Rc;

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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut current = root;
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;
        while let Some(node) = current {
            match (p_val.cmp(&node.borrow().val), q_val.cmp(&node.borrow().val)) {
                (Greater, Greater) => current = node.borrow().right.clone(),
                (Less, Less) => current = node.borrow().left.clone(),
                _ => return Some(node.clone()),
            };
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowest_common_ancestor_01() {
        assert_eq!(
            Solution::lowest_common_ancestor(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 4,
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 3,
                                left: None,
                                right: None
                            }))),
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 5,
                                left: None,
                                right: None
                            }))),
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 7,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 9,
                            left: None,
                            right: None
                        }))),
                    }))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 3,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 5,
                            left: None,
                            right: None
                        }))),
                    }))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: None
                    }))),
                }))),
            ),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 3,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 5,
                            left: None,
                            right: None
                        }))),
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: None
                    }))),
                }))),
            })))
        )
    }

    #[test]
    fn lowest_common_ancestor_02() {
        assert_eq!(
            Solution::lowest_common_ancestor(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 4,
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 3,
                                left: None,
                                right: None
                            }))),
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 5,
                                left: None,
                                right: None
                            }))),
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 7,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 9,
                            left: None,
                            right: None
                        }))),
                    }))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 3,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 5,
                            left: None,
                            right: None
                        }))),
                    }))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None
                    }))),
                }))),
            ),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None
                    }))),
                }))),
            }))),
        )
    }

    #[test]
    fn lowest_common_ancestor_03() {
        assert_eq!(
            Solution::lowest_common_ancestor(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None
                    }))),
                    right: None
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                })))
            ),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: None
            })))
        )
    }
}
