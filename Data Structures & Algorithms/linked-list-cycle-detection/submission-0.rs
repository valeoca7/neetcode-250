// Linked List Cycle Detection

// Given the beginning of a linked list head, return true if there is a cycle in the linked list. Otherwise, return false.

// There is a cycle in a linked list if at least one node in the list can be visited again by following the next pointer.

// Internally, index determines the index of the beginning of the cycle, if it exists. The tail node of the list will set it's next pointer to the index-th node. If index = -1, then the tail node points to null and no cycle exists.


// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: *mut ListNode,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: std::ptr::null_mut(), val }
//     }
// }

impl Solution {
    pub fn has_cycle(head: *mut ListNode) -> bool {
        if head.is_null() {
            return false;
        }
        let mut slow = head;
        let mut fast = head;

        unsafe {
            while !fast.is_null() && !(*fast).next.is_null() {
                slow = (*slow).next;
                fast = (*(*fast).next).next;
                if slow == fast {
                    return true;
                }
            }
        }
        false
    }
}
