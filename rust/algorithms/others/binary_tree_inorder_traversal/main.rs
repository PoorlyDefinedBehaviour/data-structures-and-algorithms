//! 94. Binary Tree Inorder Traversal
//!
//! Given the root of a binary tree, return the inorder traversal of its nodes' values.
//!
//! Example 1:
//! ┌───┐
//! │ 1 ├─────┐
//! └───┘     │
//!           │
//!         ┌─▼─┐
//!         │ 2 │
//!         └─┬─┘
//!           │
//! ┌───┐     │
//! │ 3 ◄─────┘
//! └───┘
//!
//! Input: root = [1,null,2,3]
//! Output: [1,3,2]
//!
//! Example 2:
//!
//! Input: root = []
//! Output: []
//!
//! Example 3:
//!
//! Input: root = [1]
//! Output: [1]
//!
//! Constraints:
//!
//!     The number of nodes in the tree is in the range [0, 100].
//!     -100 <= Node.val <= 100
//!
//! Follow up: Recursive solution is trivial, could you do it iteratively?

use std::{cell::RefCell, rc::Rc};

type Node = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub value: i32,
    pub left: Node,
    pub right: Node,
}

impl TreeNode {
    #[inline]
    pub fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

/// time O(n)
/// space O(n)
pub fn recursive_in_order_traversal(root: Node) -> Vec<i32> {
    fn go(node: &Node, elements: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            go(&node.left, elements);
            elements.push(node.value);
            go(&node.right, elements);
        }
    }

    let mut elements = Vec::new();

    go(&root, &mut elements);

    elements
}

/// time O(n)
/// space O(n)
pub fn iterative_in_order_traversal(root: Node) -> Vec<i32> {
    let mut stack = Vec::new();
    let mut elements = Vec::new();
    let mut current = root;

    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            stack.push(node.clone());
            current = node.borrow().left.clone();
        }

        let node = stack.pop().unwrap();
        let node = node.borrow();

        elements.push(node.value);
        current = node.right.clone();
    }

    elements
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cases() -> Vec<(Node, Vec<i32>)> {
        vec![
            (
                Some(Rc::new(RefCell::new(TreeNode {
                    value: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        value: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            value: 3,
                            left: None,
                            right: None,
                        }))),
                        right: None,
                    }))),
                }))),
                vec![1, 3, 2],
            ),
            (None, vec![]),
            (
                Some(Rc::new(RefCell::new(TreeNode {
                    value: 1,
                    left: None,
                    right: None,
                }))),
                vec![1],
            ),
        ]
    }

    #[test]
    fn test_recursive_in_order_traversal() {
        for (tree, expected) in test_cases() {
            assert_eq!(expected, recursive_in_order_traversal(tree));
        }
    }

    #[test]
    fn test_iterative_in_order_traversal() {
        for (tree, expected) in test_cases() {
            assert_eq!(expected, iterative_in_order_traversal(tree));
        }
    }
}
