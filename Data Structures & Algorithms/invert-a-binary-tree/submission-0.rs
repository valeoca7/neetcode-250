// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(node) = root else {
            return None;
        };
        let (left, right) = {
            let mut node = node.borrow_mut();
            (node.left.take(), node.right.take())
        };

        let new_left = Self::invert_tree(right);
        let new_right = Self::invert_tree(left);

        {
            let mut node = node.borrow_mut();
            node.left = new_left;
            node.right = new_right;
        }

        Some(node)
    }
}
