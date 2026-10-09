// Add Two Numbers

// You are given two non-empty linked lists, l1 and l2, where each represents a non-negative integer.

// The digits are stored in reverse order, e.g. the number 321 is represented as 1 -> 2 -> 3 -> in the linked list.

// Each of the nodes contains a single digit. You may assume the two numbers do not contain any leading zero, except the number 0 itself.

// Return the sum of the two numbers as a linked list.

// Example 1:
// Input: l1 = [1,2,3], l2 = [4,5,6]
// Output: [5,7,9]

// Explanation: 321 + 654 = 975.

// Example 2:
// Input: l1 = [9], l2 = [9]
// Output: [8,1]


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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let digit1 = Self::take_digit(&mut l1);
            let digit2 = Self::take_digit(&mut l2);

            let sum = digit1 + digit2 + carry;

            let digit = sum % 10;
            carry = sum / 10;

            tail.next = Some(Box::new(ListNode::new(digit)));

            tail = tail
                .next
                .as_mut()
                .expect("node was just inserted")
        }
        dummy.next
    }
    // Removes the current head node and returns the removed node's digit
    fn take_digit(list: &mut Option<Box<ListNode>>) -> i32 {
        match list.take() {
            Some(mut node) => {
                *list = node.next.take();
                node.val
            }
            None => 0,
        }
    }
}
