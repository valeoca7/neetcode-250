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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // pass 1
        let mut len = 0;
        let mut current = head.as_ref();

        while let Some(node) = current {
            len += 1;
            current = node.next.as_ref();
        }

        let target_index = len - n as usize;
        // Dummy node for edge case of short list
        let mut dummy = Box::new(ListNode {
            val: 0,
            next: head,
        });

        // pass 2
        let mut current = &mut dummy;
        
        for _ in 0..target_index {
            current = current
            .next
            .as_mut()
            .expect("target index must be within the list");
        }

        // actual removal the object and reconnect next node
        if let Some(mut target) = current.next.take() {
            current.next = target.next.take();
        }

        dummy.next

    }
}
