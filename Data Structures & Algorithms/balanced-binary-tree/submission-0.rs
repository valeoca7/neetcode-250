// Balanced Binary Tree

// Given a binary tree, return true if it is height-balanced and false otherwise.

// A height-balanced binary tree is defined as a binary tree in which the left and right subtrees of every node differ in height by no more than 1.

// Example 1:
// Input: root = [1,2,3,null,null,4]
// Output: true

// Example 2:
// Input: root = [1,2,3,null,null,4,null,5]
// Output: false

// Example 3:
// Input: root = []
// Output: true

// Constraints:

// The number of nodes in the tree is in the range [0, 1000].
// -1000 <= Node.val <= 1000

//////////////////////////////////////////////////////////////////////////

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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::height(root) >= 0
    }

    pub fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(node) = root else {
            return 0;
        };
        let (left, right) = {
            let node = node.borrow();
            (node.left.clone(), node.right.clone())
        };
        let left_height = Self::height(left);
        if left_height < 0 {
            return -1;
        }
        let right_height = Self::height(right);
        if right_height < 0 {
            return -1;
        }
        if (left_height - right_height).abs() > 1 {
            return -1;
        }
        1 + left_height.max(right_height)
    }
}
