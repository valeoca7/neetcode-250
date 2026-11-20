// Count Good Nodes in Binary Tree

// Within a binary tree, a node x is considered good if the path from the root of the tree to the node x contains no nodes with a value greater than the value of node x

// Given the root of a binary tree root, return the number of good nodes within the tree.

// Example 1:
// Input: root = [2,1,1,3,null,1,5]
// Output: 3

// Example 2:
// Input: root = [1,2,-1,3,4]
// Output: 4

// Constraints:
// 1 <= number of nodes in the tree <= 100,000
// -100 <= Node.val <= 100

// Recommended Time & Space Complexity
// You should aim for a solution with O(n) time andO(n) space, where n is the number of nodes in the given tree.

/////////////////////////////////////////////////////////////////////////////

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
    pub fn good_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> i32 {
        let Some(root) = root else {
            return 0;
        };

        let mut stack = vec![(root, i32::MIN)];
        let mut count = 0;

        while let Some((node, max_so_far)) = stack.pop() {
            let node = node.borrow();

            if node.val >= max_so_far {
                count += 1;
            }

            let next_max = max_so_far.max(node.val);

            if let Some(left) = &node.left {
                stack.push((Rc::clone(left), next_max));
            }

            if let Some(right) = &node.right {
                stack.push((Rc::clone(right), next_max));
            }
        }

        count
    }
}