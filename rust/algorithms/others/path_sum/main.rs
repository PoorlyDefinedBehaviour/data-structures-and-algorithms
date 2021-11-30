use std::{cell::RefCell, rc::Rc};

// 112. Path Sum
//
// Given the root of a binary tree and an integer targetSum,
// return true if the tree has a root-to-leaf path such that adding up all the values along the path equals targetSum.
//
// A leaf is a node with no children.
//
// Example 1:
//
// Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
// Output: true
// Explanation: The root-to-leaf path with the target sum is shown.
//
// Example 2:
//
// Input: root = [1,2,3], targetSum = 5
// Output: false
// Explanation: There two root-to-leaf paths in the tree:
// (1 --> 2): The sum is 3.
// (1 --> 3): The sum is 4.
// There is no root-to-leaf path with sum = 5.
//
// Example 3:
//
// Input: root = [], targetSum = 0
// Output: false
// Explanation: Since the tree is empty, there are no root-to-leaf paths.
//
// Constraints:
//
//     The number of nodes in the tree is in the range [0, 5000].
//     -1000 <= Node.val <= 1000
//     -1000 <= targetSum <= 1000

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn is_leaf(&self) -> bool {
    self.left.is_none() && self.right.is_none()
  }
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None,
    }
  }
}

// time O(n) where n is the height of the tree
// space O(n) where n is the height of the tree
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
  fn go(node: Option<Rc<RefCell<TreeNode>>>, current_sum: i32, target_sum: i32) -> bool {
    match node {
      None => false,
      Some(node) => {
        let node = node.borrow();

        let current_sum = current_sum + node.val;

        if node.is_leaf() {
          current_sum == target_sum
        } else {
          go(node.left.clone(), current_sum, target_sum)
            || go(node.right.clone(), current_sum, target_sum)
        }
      }
    }
  }

  go(root, 0, target_sum)
}

fn main() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_path_sum() {
    let tests = vec![
      (
        Some(Rc::new(RefCell::new(TreeNode {
          val: 5,
          left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
              val: 11,
              right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
              }))),
              left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
              }))),
            }))),
            right: None,
          }))),
          right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
              val: 13,
              left: None,
              right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
              val: 4,
              left: None,
              right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
              }))),
            }))),
          }))),
        }))),
        22,
        true,
      ),
      (
        Some(Rc::new(RefCell::new(TreeNode {
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
        }))),
        5,
        false,
      ),
      (None, 0, false),
    ];

    for (input, target_sum, expected) in tests {
      assert_eq!(expected, has_path_sum(input, target_sum));
    }
  }
}
