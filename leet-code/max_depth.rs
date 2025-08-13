use std::{cell::RefCell, rc::Rc};
use std::cmp::max;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let root = root.unwrap();
    let mut depth = 1;
    let left_depth = max_depth(root.borrow().left.clone());
    let right_depth = max_depth(root.borrow().right.clone());
    depth + max(left_depth, right_depth)
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })));

    let depth = max_depth(root);
    println!("Max depth of the tree is: {}", depth); // Should print: Max depth of the tree is: 2
}

