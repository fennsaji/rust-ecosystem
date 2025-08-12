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
    let mut pointer1 = list1;
    let mut pointer2 = list2;
    let mut newlist: Option<Box<ListNode>> = None;
    
    while pointer1.is_some() || pointer2.is_some() {
        match (pointer1.clone(), pointer2.clone()) {
            (Some(p1), Some(p2)) => {
                if p1.val < p2.val {
                    newlist = Some(Box::new(ListNode {
                        val: p1.val,
                        next: newlist,
                    }));
                    pointer1 = p1.next;
                } else {
                    newlist = Some(Box::new(ListNode {
                        val: p2.val,
                        next: newlist,
                    }));
                    pointer2 = p2.next;
                }
            },
            (Some(p1), None) => {
                newlist = Some(Box::new(ListNode {
                    val: p1.val,
                    next: newlist,
                }));
                pointer1 = p1.next;
            },
            (None, Some(p2)) => {
                newlist = Some(Box::new(ListNode {
                    val: p2.val,
                    next: newlist,
                }));
                pointer2 = p2.next;
            },
            (None, None) => {
                break;
            }
        }
    }
    reverse_list(newlist)
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