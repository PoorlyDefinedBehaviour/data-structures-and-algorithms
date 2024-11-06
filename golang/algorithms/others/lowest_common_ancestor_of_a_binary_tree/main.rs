use std::{cell::RefCell, collections::HashSet, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[derive(PartialEq, Eq)]
struct HashableTreeNode(Rc<RefCell<TreeNode>>);

impl std::hash::Hash for HashableTreeNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.borrow().val.hash(state);
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

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let ancestors_p = find_ancestors(root.clone(), p.unwrap().borrow().val, Vec::new()).unwrap();
    let ancestors_q = find_ancestors(root, q.unwrap().borrow().val, Vec::new()).unwrap();

    let ancestors_p: HashSet<_> = ancestors_p.into_iter().map(HashableTreeNode).collect();

    for ancestor in ancestors_q.into_iter().rev() {
        if ancestors_p.contains(&HashableTreeNode(Rc::clone(&ancestor))) {
            return Some(ancestor);
        }
    }

    None
}

pub fn find_ancestors(
    root: Option<Rc<RefCell<TreeNode>>>,
    v: i32,
    mut ancestors: Vec<Rc<RefCell<TreeNode>>>,
) -> Option<Vec<Rc<RefCell<TreeNode>>>> {
    match root {
        None => None,
        Some(node) => {
            ancestors.push(Rc::clone(&node));
            if node.borrow().val == v {
                return Some(ancestors);
            }

            if let Some(ancestors) =
                find_ancestors(node.borrow().left.clone(), v, ancestors.clone())
            {
                return Some(ancestors);
            }

            find_ancestors(node.borrow().right.clone(), v, ancestors)
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowest_common_ancestor() {
        let tree_1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let result = lowest_common_ancestor(
            tree_1.clone(),
            tree_1.clone().and_then(|tree| tree.borrow().left.clone()),
            tree_1.clone().and_then(|tree| tree.borrow().right.clone()),
        );
        assert_eq!(3, result.unwrap().borrow().val);

        let result = lowest_common_ancestor(
            tree_1.clone(),
            tree_1.clone().and_then(|tree| tree.borrow().left.clone()),
            tree_1
                .and_then(|tree| tree.borrow().left.clone())
                .and_then(|node| node.borrow().right.clone())
                .and_then(|node| node.borrow().right.clone()),
        );
        assert_eq!(5, result.unwrap().borrow().val);

        let tree_2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: None,
        })));
        let result = lowest_common_ancestor(
            tree_2.clone(),
            tree_2.clone(),
            tree_2.and_then(|tree| tree.borrow().left.clone()),
        );
        assert_eq!(1, result.unwrap().borrow().val);
    }
}
