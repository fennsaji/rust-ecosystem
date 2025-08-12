// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Create a dummy head node to simplify edge cases
    // This avoids having to handle the first node separately
    let mut dummy = Box::new(ListNode { val: 0, next: None });
    
    // Keep a mutable reference to the last node in our result list
    // This allows us to append nodes efficiently
    let mut tail = &mut dummy;
    
    // Take ownership of the input lists so we can modify them
    let mut l1 = list1;
    let mut l2 = list2;
    
    // Compare and merge nodes while both lists have elements
    while l1.is_some() && l2.is_some() {
        // Compare the values of the current nodes
        if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
            // l1's current node is smaller or equal, so add it to result
            tail.next = l1.take();           // Move l1's current node to result
            tail = tail.next.as_mut().unwrap(); // Advance tail to the new last node
            l1 = tail.next.take();           // Move l1 to its next node
        } else {
            // l2's current node is smaller, so add it to result
            tail.next = l2.take();           // Move l2's current node to result
            tail = tail.next.as_mut().unwrap(); // Advance tail to the new last node
            l2 = tail.next.take();           // Move l2 to its next node
        }
    }
    
    // Append any remaining nodes from either list
    // Only one of l1 or l2 can have remaining elements at this point
    tail.next = l1.or(l2);
    
    // Return the merged list, skipping the dummy head
    dummy.next
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut new_head: Option<Box<ListNode>> = None;
    let mut head = head.clone();
    while head != None {
        new_head = Some(Box::new(ListNode {
            val: head.as_ref().unwrap().val,
            next: new_head,
        }));
        head = head.unwrap().next;
    }
    new_head
}

fn main() {
    let list1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let merged = merge_two_lists(list1, list2);
    // Print the merged list
    let mut current = merged;
    while let Some(node) = current {
        print!("{} ", node.val);
        current = node.next;
    }
}