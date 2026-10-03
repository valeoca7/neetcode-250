// You are given the head of a singly linked-list.
// The positions of a linked list of length = 7 for example, can intially be represented as:
// [0, 1, 2, 3, 4, 5, 6]
// Reorder the nodes of the linked list to be in the following order:
// [0, 6, 1, 5, 2, 4, 3]
// In the general case, label the nodes by their original zero-based positions from 0 to n - 1. After reordering, those original positions appear in this order:
// [0, n-1, 1, n-2, 2, n-3, ...]
// These numbers represent node positions, not the values stored in the nodes.
// You may not modify the values in the list's nodes, but instead you must reorder the nodes themselves.


// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }


impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.as_ref().and_then(|node| node.next.as_ref()).is_none() {
            return;
        }
        // get split point
        let len = {
            let mut len = 0;
            let mut cursor = head.as_ref();
            while let Some(node) = cursor {
                len += 1;
                cursor = node.next.as_ref();
            }
            len
        };
        let first_len = (len + 1) / 2;

        // split the list
        let mut split = head.as_mut().expect("list is non-empty");
        for _ in 1..first_len {
            split = split
                .next
                .as_mut()
                .expect("split point must be within the list");
        }
        let second = split.next.take();

        // reverse second half of list
        let mut second = Self::reverse(second);
        let mut cursor = head;

        while let Some(mut second_node) = second {
            second = second_node.next.take();
            let first_node = cursor
                .as_mut()
                .expect("first half cannot be shorter then second half");
            let next_first = first_node.next.take();
            second_node.next = next_first;
            first_node.next = Some(second_node);
            cursor = &mut first_node
                .next
                .as_mut()
                .expect("second node was just inserted")
                .next;
        }
    }

    pub fn reverse (mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut previous = None;
        while let Some(mut current) = head {
            head = current.next.take();
            current.next = previous;
            previous = Some(current);
        }
        previous
    }
}
