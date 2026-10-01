// You are given the heads of two sorted linked lists list1 and list2.
// Merge the two lists into one sorted linked list and return the head of the new sorted linked list.
// The new list should be made up of nodes from list1 and list2.

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
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while list1.is_some() && list2.is_some() {
            let take_list1 = list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val;
            let mut node = if take_list1 {
                let mut node = list1.take().unwrap();
                list1 = node.next.take();
                node
            } else {
                let mut node = list2.take().unwrap();
                list2 = node.next.take();
                node
            };
            node.next = None;
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if list1.is_some() {
            list1
        } else {
            list2
        };

        dummy.next
    }
}

