// Binary Tree Right Side View

// You are given the root of a binary tree. Return only the values of the nodes that are visible from the right side of the tree, ordered from top to bottom.

// Example 1:
// Input: root = [1,2,3,null,4,null,5]
// Output: [1,3,5]

// Example 2:
// Input: root = [1,2,3,4,null,null,null,5]
// Output: [1,3,4,5]

// Example 3:
// Input: root = [1,null,2]
// Output: [1,2]

// Example 4:
// Input: root = []
// Output: []

// Constraints:
// 0 <= number of nodes in the tree <= 100
// -100 <= Node.val <= 100

// Recommended Time & Space Complexity
// You should aim for a solution with O(n) time and O(n) space, where n is the number of nodes in the given tree.

////////////////////////////////////////////////////////////////////

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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut result = Vec::new();
        Self::dfs(root, 0, &mut result);
        result
    }

    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        depth: usize,
        result: &mut Vec<i32>,
    ) {
        let Some(node) = root else {
            return;
        };

        let (val, left, right) = {
            let node = node.borrow();
            (node.val, node.left.clone(), node.right.clone())
        };

        if depth == result.len() {
            result.push(val);
        }

        // Visit right subtree first to prioritize visible nodes.
        Self::dfs(right, depth + 1, result);
        Self::dfs(left, depth + 1, result);
    }
}