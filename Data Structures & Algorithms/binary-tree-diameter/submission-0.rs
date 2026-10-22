// Diameter of Binary Tree

// The diameter of a binary tree is defined as the length of the longest path between any two nodes within the tree. The path does not necessarily have to pass through the root.

// The length of a path between two nodes in a binary tree is the number of edges between the nodes. Note that the path can not include the same node twice.

// Given the root of a binary tree root, return the diameter of the tree.

// Example 1:
// Input: root = [1,null,2,3,4,5]
// Output: 3
// Explanation: 3 is the length of the path [1,2,3,5] or [5,3,2,4].

// Example 2:
// Input: root = [1,2,3]
// Output: 2

// Constraints:
// 1 <= number of nodes in the tree <= 100
// -100 <= Node.val <= 100

// Recommended Time & Space Complexity
// You should aim for a solution with O(n) time and O(n) space, where n is the number of nodes in the tree.

//////////////////////////////////////////////////////////////////////////////////////

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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut diameter = 0;
        Self::height(root, &mut diameter);
        diameter
    }

    fn height(root: Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
        let Some(node) = root else {
            return 0;
        };
        let (left, right) = {
            let node = node.borrow();
            (node.left.clone(), node.right.clone())
        };
        let left_height = Self::height(left, diameter);
        let right_height = Self::height(right, diameter);

        // Number of edges on the longest path
        *diameter = (*diameter).max(left_height + right_height);

        // Height of this subtree (nodes)
        1 + left_height.max(right_height)
    }
}
