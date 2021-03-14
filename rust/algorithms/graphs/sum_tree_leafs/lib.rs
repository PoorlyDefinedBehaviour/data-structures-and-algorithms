pub enum BinaryTree<T> {
  Leaf(T),
  Node(Box<BinaryTree<T>>, Box<BinaryTree<T>>),
}

pub fn sum_leafs(tree: &BinaryTree<i32>) -> i32 {
  use BinaryTree::*;
  match tree {
    Leaf(x) => *x,
    Node(left, right) => sum_leafs(left) + sum_leafs(right),
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
        10,
      ),
      (Leaf(1), 1),
      (Leaf(0), 0),
      (
        Node(
          Box::new(Node(Box::new(Leaf(-3)), Box::new(Leaf(-4)))),
          Box::new(Node(Box::new(Leaf(3)), Box::new(Leaf(4)))),
        ),
        0,
      ),
    ];

    for (tree, expected) in test_cases {
      let actual = sum_leafs(&tree);

      assert_eq!(expected, actual);
    }
  }
}
