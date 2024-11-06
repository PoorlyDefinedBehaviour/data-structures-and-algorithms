use std::{cell::RefCell, rc::Rc};

/// Given the root of a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.
///
/// Example 1:
//
/// Input: root = [1,2,3,null,5,null,4]
/// Output: [1,3,4]
/// Example 2:
//
/// Input: root = [1,null,3]
/// Output: [1,3]
/// Example 3:
//
/// Input: root = []
/// Output: []
//
/// Constraints:
//
/// The number of nodes in the tree is in the range [0, 100].
/// -100 <= Node.val <= 100

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut nodes = Vec::new();
    do_right_side_view(root, &mut nodes, 0, &mut 0);
    nodes
}

fn do_right_side_view(
    root: Option<Rc<RefCell<TreeNode>>>,
    nodes: &mut Vec<i32>,
    depth: usize,
    visited_depth: &mut usize,
) {
    if let Some(node) = root {
        if depth == *visited_depth {
            nodes.push(node.borrow().val);
            *visited_depth += 1;
        }
        do_right_side_view(node.borrow().right.clone(), nodes, depth + 1, visited_depth);
        do_right_side_view(node.borrow().left.clone(), nodes, depth + 1, visited_depth);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_side_view() {
        let tree_1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(vec![1, 3], right_side_view(tree_1));

        let tree_2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(vec![1, 3, 4], right_side_view(tree_2));
    }
}
