// Definition for a Node.
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct Node {
//     pub val: i32,
//     pub next: Option<Rc<RefCell<Node>>>,
//     pub random: Option<Rc<RefCell<Node>>>,
// }
//
// impl Node {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         Node {
//             val,
//             next: None,
//             random: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let Some(head) = head else {
            return None;
        };
        let mut copies: HashMap<*const RefCell<Node>, Rc<RefCell<Node>>,> = HashMap::new();
        
        // pass 1
        let mut current = Some(Rc::clone(&head));
        while let Some(node) = current {
            let (val, next) = {
                let node = node.borrow();
                (node.val, node.next.clone())
            };
            copies.insert(
                Rc::as_ptr(&node),
                Rc::new(RefCell::new(Node::new(val))),
            );
            current = next;
        }
        // pass 2
        let mut current = Some(Rc::clone(&head));
        while let Some(node) = current {
            let (next, random) = {
                let node = node.borrow();
                (node.next.clone(), node.random.clone())
            };
            let copied_node = copies
                .get(&Rc::as_ptr(&node))
                .expect("every ogiginal node must have a copy");
            let copied_next = next
                .as_ref()
                .map(|next| {
                    Rc::clone(
                        copies
                            .get(&Rc::as_ptr(next))
                            .expect("next must reference a node in the list")
                    )
                });
            let copied_random = random
                .as_ref()
                .map(|random| {
                    Rc::clone(
                        copies
                            .get(&Rc::as_ptr(random))
                            .expect("random must reference a node in the list")
                    )
                });
            {
                let mut copy = copied_node.borrow_mut();
                copy.next = copied_next;
                copy.random = copied_random;
            }
            current = next;
        }

        copies
            .get(&Rc::as_ptr(&head))
            .cloned()

    }
}
