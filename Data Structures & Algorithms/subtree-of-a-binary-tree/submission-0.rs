// Subtree of Another Tree

// Given the roots of two binary trees root and subRoot, return true if there is a subtree of root with the same structure and node values of subRoot and false otherwise.

// A subtree of a binary tree tree is a tree that consists of a node in tree and all of this node's descendants. The tree tree could also be considered as a subtree of itself.

// Example 1:
// Input: root = [1,2,3,4,5], subRoot = [2,4,5]
// Output: true

// Example 2:
// Input: root = [1,2,3,4,5,null,null,6], subRoot = [2,4,5]
// Output: false

// Constraints:

// The number of nodes in the root tree is in the range [1, 2000].
// The number of nodes in the subRoot tree is in the range [1, 1000].
// -10^4 <= root.val <= 10^4
// -10^4 <= subRoot.val <= 10^4

// Recommended Time & Space Complexity
// You should aim for a solution as good or better than O(m * n) time and O(m + n) space, where n and m are the number of nodes in root and subRoot, respectively.

///////////////////////////////////////////////////////////////////////////////////

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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let Some(node) = root else {
            return sub_root.is_none();
        };
        if Self::same_tree(Some(node.clone()), sub_root.clone()) {
            return true;
        }
        let (left, right) = {
            let node = node.borrow();
            (node.left.clone(), node.right.clone())
        };
        Self::is_subtree(left, sub_root.clone())
            || Self::is_subtree(right, sub_root)
    }
    fn same_tree(
        a: Option<Rc<RefCell<TreeNode>>>,
        b: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                let a = a.borrow();
                let b = b.borrow();
                a.val == b.val
                    && Self::same_tree(a.left.clone(), b.left.clone())
                    && Self::same_tree(a.right.clone(), b.right.clone())
            }
            _ => false,
        }
    }
}
