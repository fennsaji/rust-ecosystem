use std::rc::Rc;
use std::cell::RefCell;

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

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
        if let Some(n) = node {
            let n = n.borrow();
            let left = dfs(n.left.clone(), res);
            let right = dfs(n.right.clone(), res);
            *res = (*res).max(left + right);
            return 1 + left.max(right);
        }
        0
    }

    dfs(root, &mut res);
    res
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })));

    let diameter = diameter_of_binary_tree(root);
    println!("Diameter of the binary tree is: {}", diameter); // Should print the diameter
}