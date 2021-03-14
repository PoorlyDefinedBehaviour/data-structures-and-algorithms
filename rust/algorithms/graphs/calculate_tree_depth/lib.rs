use std::cmp::max;

pub enum BinaryTree<T> {
  Leaf(T),
  Node(Box<BinaryTree<T>>, Box<BinaryTree<T>>),
}

pub fn depth(tree: &BinaryTree<i32>) -> i32 {
  use BinaryTree::*;
  match tree {
    Leaf(_) => 1,
    Node(left, right) => max(depth(left), depth(right)) + 1,
  }
}

fn main() {}

#[cfg(test)]
mod tests {
  use super::*;
  use BinaryTree::*;

  #[test]
  fn test() {
    let test_cases = vec![
      (
        Node(
          Box::new(Node(Box::new(Leaf(1)), Box::new(Leaf(2)))),
          Box::new(Node(Box::new(Leaf(3)), Box::new(Leaf(4)))),
        ),
        3,
      ),
      (Leaf(1), 1),
      (Leaf(0), 1),
      (
        Node(
          Box::new(Leaf(0)),
          Box::new(Node(
            Box::new(Node(Box::new(Leaf(1)), Box::new(Leaf(2)))),
            Box::new(Leaf(-4)),
          )),
        ),
        4,
      ),
    ];

    for (tree, expected) in test_cases {
      let actual = depth(&tree);

      assert_eq!(expected, actual);
    }
  }
}
