// Same Binary Tree

// Given the roots of two binary trees p and q, return true if the trees are equivalent, otherwise return false.

// Two binary trees are considered equivalent if they share the exact same structure and the nodes have the same values.

// Example 1:
// Input: p = [1,2,3], q = [1,2,3]
// Output: true

// Example 2:
// Input: p = [4,7], q = [4,null,7]
// Output: false

// Example 3:
// Input: p = [1,2,3], q = [1,3,2]
// Output: false

// Constraints:
// 0 <= The number of nodes in both trees <= 100.
// -100 <= Node.val <= 100

// Recommended Time & Space Complexity
// You should aim for a solution with O(n) time and O(n) space, where n is the number of nodes in the tree.

//////////////////////////////////////////////////////////////////////////////

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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();
                p.val == q.val
                    && Self::is_same_tree(p.left.clone(), q.left.clone())
                    && Self::is_same_tree(p.right.clone(), q.right.clone())
            }
        }
    }
}
