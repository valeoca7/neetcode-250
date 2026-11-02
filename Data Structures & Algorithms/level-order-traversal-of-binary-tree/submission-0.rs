// Binary Tree Level Order Traversal

// Given a binary tree root, return the level order traversal of it as a nested list, where each sublist contains the values of nodes at a particular level in the tree, from left to right.

// Example 1:
// Input: root = [1,2,3,4,5,6,7]
// Output: [[1],[2,3],[4,5,6,7]]

// Example 2:
// Input: root = [1]
// Output: [[1]]

// Example 3:
// Input: root = []
// Output: []

// Constraints:
// 0 <= The number of nodes in the tree <= 2000.
// -1000 <= Node.val <= 1000

// Recommended Time & Space Complexity
// You should aim for a solution with O(n) time and O(n) space, where n is the number of nodes in the given tree.

/////////////////////////////////////////////////////////////////

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
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }

impl Solution {
    pub fn level_order(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Vec<i32>> {
        let Some(root) = root else {
            return Vec::new();
        };

        let mut result = Vec::new();
        let mut queue = VecDeque::from([root]);

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut level = Vec::with_capacity(level_size);

            for _ in 0..level_size {
                let node = queue
                    .pop_front()
                    .expect("queue contains the current level");

                let node = node.borrow();
                level.push(node.val);

                if let Some(left) = &node.left {
                    queue.push_back(Rc::clone(left));
                }

                if let Some(right) = &node.right {
                    queue.push_back(Rc::clone(right));
                }
            }

            result.push(level);
        }

        result
    }
}