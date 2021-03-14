use std::cmp::Eq;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq, Clone)]
pub enum Tree<T> {
  Leaf,
  Children(Vec<Tree<T>>),
  Node(T, Box<Tree<T>>),
}

type AdjencyList<T> = HashMap<T, Vec<T>>;

pub fn adjency_list_to_rooted_tree<T: Eq + Hash + Clone>(
  root_node: T,
  adjency_list: &AdjencyList<T>,
) -> Tree<T> {
  use Tree::*;

  fn go<T: Eq + Hash + Clone>(
    current_node: &T,
    tree: &Tree<T>,
    adjency_list: &AdjencyList<T>,
    visited_nodes: &mut HashSet<T>,
  ) -> Tree<T> {
    if visited_nodes.contains(current_node) {
      return Leaf;
    }

    visited_nodes.insert(current_node.clone());

    let neighbors = adjency_list.get(current_node).unwrap();

    let neighbor_trees: Vec<Tree<T>> = neighbors
      .iter()
      .map(|neighbor| go(neighbor, tree, adjency_list, visited_nodes))
      .filter(|neighbor_tree| match neighbor_tree {
        Leaf => false,
        _ => true,
      })
      .collect();

    if neighbor_trees.is_empty() {
      Node(current_node.clone(), Box::new(Leaf))
    } else {
      Node(current_node.clone(), Box::new(Children(neighbor_trees)))
    }
  };

  let mut visited_nodes = HashSet::new();

  go(&root_node, &Leaf, adjency_list, &mut visited_nodes)
}

fn main() {}

#[cfg(test)]
mod tests {
  use super::*;
  use Tree::*;

  macro_rules! hashmap {
    ($($key: expr => $value: expr), *) => {{
        let mut map = HashMap::new();
        $(map.insert($key, $value);)*
        map
    }};
  }

  #[test]
  fn test() {
    let test_cases = vec![(
      0,
      hashmap! {
        0 => vec![1,2,5],
        1 => vec![0],
        2 => vec![3,0],
        3 => vec![2],
        4 => vec![5],
        5 => vec![0,6,4],
        6 => vec![5]
      },
      /*
              0
            / | \
           /  |  \
          5   1   2
        /  \      |
       6    4     3
      */
      Node(
        0,
        Box::new(Children(vec![
          Node(1, Box::new(Leaf)),
          Node(2, Box::new(Children(vec![Node(3, Box::new(Leaf))]))),
          Node(
            5,
            Box::new(Children(vec![
              Node(6, Box::new(Leaf)),
              Node(4, Box::new(Leaf)),
            ])),
          ),
        ])),
      ),
    )];

    for (root_node, input, expected) in test_cases {
      let actual = adjency_list_to_rooted_tree(root_node, &input);

      assert_eq!(expected, actual);
    }
  }
}
