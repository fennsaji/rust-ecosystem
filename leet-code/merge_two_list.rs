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

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut pointer1 = list1;
    let mut pointer2 = list2;

    let mut dummy = Box::new(ListNode { val: 0, next: None });
    let mut tail = &mut dummy;

    while pointer1.is_some() || pointer2.is_some() {
        match (pointer1.take(), pointer2.take()) {
            (Some(mut p1), Some(mut p2)) => {
                if p1.val < p2.val {
                    pointer1 = p1.next.take();
                    pointer2 = Some(p2);
                    tail.next = Some(p1);
                } else {
                    pointer2 = p2.next.take();
                    pointer1 = Some(p1);
                    tail.next = Some(p2);
                }
                tail = tail.next.as_mut().unwrap();
            }
            (Some(mut p1), None) => {
                pointer1 = p1.next.take();
                tail.next = Some(p1);
                tail = tail.next.as_mut().unwrap();
            }
            (None, Some(mut p2)) => {
                pointer2 = p2.next.take();
                tail.next = Some(p2);
                tail = tail.next.as_mut().unwrap();
            }
            (None, None) => break,
        }
    }

    dummy.next
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