// Lowest Common Ancestor in Binary Search Tree

// Given a binary search tree (BST) where all node values are unique, and two nodes from the tree p and q, return the lowest common ancestor (LCA) of the two nodes.

// The lowest common ancestor between two nodes p and q is the lowest node in a tree T such that both p and q are descendants. The ancestor is allowed to be a descendant of itself.

// Example 1:
// Input: root = [5,3,8,1,4,7,9,null,2], p = 3, q = 8
// Output: 5

// Example 2:
// Input: root = [5,3,8,1,4,7,9,null,2], p = 3, q = 4
// Output: 3
// Explanation: The LCA of nodes 3 and 4 is 3, since a node can be a descendant of itself.

// Constraints:
// 2 <= The number of nodes in the tree <= 100.
// -100 <= Node.val <= 100
// p != q
// p and q will both exist in the BST.

// Recommended Time & Space Complexity
// You should aim for a solution as good or better than O(h) time and O(h) space, where h is the height of the given tree.

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

use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p?.borrow().val;
        let q_val = q?.borrow().val;

        let (low, high) = if p_val < q_val {
            (p_val, q_val)
        } else {
            (q_val, p_val)
        };

        let mut current = root;

        while let Some(node) = current {
            let (val, left, right) = {
                let node = node.borrow();
                (node.val, node.left.clone(), node.right.clone())
            };

            if high < val {
                current = left;
            } else if low > val {
                current = right;
            } else {
                return Some(node);
            }
        }

        None
    }
}
