// Maximum Depth of Binary Tree

// Given the root of a binary tree, return its depth.

// The depth of a binary tree is defined as the number of nodes along the longest path from the root node down to the farthest leaf node.

// Example 1:
// Input: root = [1,2,3,null,null,4]
// Output: 3

// Example 2:
// Input: root = []
// Output: 0

// Constraints:

// 0 <= The number of nodes in the tree <= 100.
// -100 <= Node.val <= 100

// Recommended Time & Space Complexity
// You should aim for a solution with O(n) time and O(n) space, where n is the number of nodes in the tree.

///////////////////////////////////////////////////////////////

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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(node) = root else {
            return 0;
        };

        let (left, right) = {
            let node = node.borrow();
            (node.left.clone(), node.right.clone())
        };
        1 + Self::max_depth(left).max(Self::max_depth(right))
    }
}
