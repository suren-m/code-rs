# Add two numbers LinkedList
[https://leetcode.com/problems/add-two-numbers/](https://leetcode.com/problems/add-two-numbers/)

```rust
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // 2 4 3            9 9 9 9 9 9 9
    // 5 6 4            9 9 9 9
    // 7 0 8            8 9 9 9 0 0 0 1 (remain carry taken as it is)
    // 0 0 1 (carry)    0 1 1 1 1 1 1 1

    // return head of the result list
    // keep a mutable carry
    // make lists mutable
    // carry from left instead of right (no reversing needed)
    let mut dummy_head = ListNode::new(0);
    // make current point to dummy_head
    let mut current = &mut dummy_head;
    let mut carry = 0;
    let mut l1 = l1;
    let mut l2 = l2;

    // loop until either of the list is not None
    while l1.is_some() || l2.is_some() {
        // check if both lists are valid or just one
        // pass ref to match since we are in a loop. (can't move)
        let add_res = match (&l1, &l2) {
            // perform add
            (Some(a), Some(b)) => a.val + b.val,
            (Some(a), None) => a.val,
            (None, Some(b)) => b.val,
            _ => 0, // can't have None, None
        } + carry;
        carry = add_res / 10; // remainder from result.
        let mut sum = add_res % 10;

        // set current.next to new node with current val
        // not setting to current.val because then we can't loop
        // effectively, in first iter setting dummy_head.next to actual head.
        // current has a mutable ref to dummy_head
        current.next = Some(Box::new(ListNode::new(sum)));

        // now update current itself, so the state is kept in next iteration.
        current = current.next.as_mut().unwrap();

        // update l1 and l2 to next if they are currently valid.
        // for e.g: if current l1 is None, then we've reached the end of l1
        l1 = if l1.is_some() { l1.unwrap().next } else { None };
        l2 = if l2.is_some() { l2.unwrap().next } else { None };
    }

    // if the last set of add also returns a carry
    if carry > 0 {
        current.next = Some(Box::new(ListNode::new(carry)));
    }

    dummy_head.next // actual head begins on next of dummy
}

let mut l1_head = ListNode::new(2);
 let mut l1_2nd_item = ListNode::new(4);
 let l1_tail = ListNode::new(3);
 l1_2nd_item.next = Some(Box::new(l1_tail));
 l1_head.next = Some(Box::new(l1_2nd_item));

 let mut l2_head = ListNode::new(5);
 let mut l2_2nd_item = ListNode::new(6);
 let l2_tail = ListNode::new(4);
 l2_2nd_item.next = Some(Box::new(l2_tail));
 l2_head.next = Some(Box::new(l2_2nd_item));

 let l1 = Some(Box::new(l1_head));
 let l2 = Some(Box::new(l2_head));

 let mut expected_head = ListNode::new(7);
 let mut expected_2nd_item = ListNode::new(0);
 let expected_tail = ListNode::new(8);
 expected_2nd_item.next = Some(Box::new(expected_tail));
 expected_head.next = Some(Box::new(expected_2nd_item));

 let expected = Some(Box::new(expected_head));

 let actual = add_two_numbers(l1, l2);
 assert_eq!(expected, actual);
```