use std::fmt::Debug;

enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

fn in_order<T>(tree: &Tree<T>)
where
    T: Debug,
{
    if let Tree::Node(left, value, right) = tree {
        in_order(left);
        println!("{:?}", value);
        in_order(right);
    }
}

fn pre_order<T>(tree: &Tree<T>)
where
    T: Debug,
{
    if let Tree::Node(left, value, right) = tree {
        println!("{:?}", value);
        pre_order(left);
        pre_order(right);
    }
}

fn post_order<T>(tree: &Tree<T>)
where
    T: Debug,
{
    if let Tree::Node(left, value, right) = tree {
        post_order(left);
        post_order(right);
        println!("{:?}", value);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
      #[test]
      fn simple(x: i32) {

      }
    }
}
