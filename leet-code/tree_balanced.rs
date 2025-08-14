use std::{cell::RefCell, rc::Rc};

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

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
        if let Some(root_rc) = root {
            let (left_height, left_balanced) = dfs(root_rc.borrow().left.clone());
            let (right_height, right_balanced) = dfs(root_rc.borrow().right.clone());
            let difference = right_height - left_height;
            let is_balanced =  if difference.abs() <= 1 && left_balanced && right_balanced {
                true
            } else {
                false
            };
            return (1 + left_height.max(right_height), is_balanced);
        }
        return (0, true);
    }
    let (_, is_balanced) = dfs(root);
    is_balanced
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        }))),
    })));

    let balanced = is_balanced(root);
    println!("Is the tree balanced? {}", balanced); // Should print: Is the tree balanced? false

    // Unbalanced tree
    let unbalanced_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                right: None,
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        }))),
    })));

    let balanced = is_balanced(unbalanced_root);
    println!("Is the tree balanced? {}", balanced); // Should print: Is the tree balanced? false
}