use std::collections::HashSet;
use std::collections::LinkedList;

pub type Graph<Representation> = Representation;

pub type AdjencyList = Vec<Vec<i32>>;

/**
 * time O(V + E)
 * space O(2E)
 */
pub fn breadth_first_search(node: i32, graph: &Graph<AdjencyList>) -> bool {
  let mut queue = LinkedList::new();

  let mut visited_nodes = HashSet::new();

  // O(1)
  queue.push_back(0);

  while !queue.is_empty() {
    // O(1)
    let current_node = queue.pop_front().unwrap();

    if current_node == node {
      return true;
    }

    // O(1)~*
    visited_nodes.insert(current_node);

    for neighbor in &graph[current_node as usize] {
      // O(1)
      if !visited_nodes.contains(neighbor) {
        // O(1)
        queue.push_back(*neighbor);
      }
    }
  }

  return false;
}

fn main() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn search() {
    /*
        1 - 3 - 5
      /        /
    0        /
      \    /
        2 - 4 - 6
      */
    let test_cases = vec![
      (
        5,
        vec![
          vec![1, 2],
          vec![0, 3],
          vec![0, 4],
          vec![1, 5],
          vec![2, 6],
          vec![3, 4],
          vec![6, 4],
        ],
        true,
      ),
      (
        32,
        vec![
          vec![1, 2],
          vec![0, 3],
          vec![0, 4],
          vec![1, 5],
          vec![2, 6],
          vec![3, 4],
          vec![6, 4],
        ],
        false,
      ),
      (
        0,
        vec![
          vec![1, 2],
          vec![0, 3],
          vec![0, 4],
          vec![1, 5],
          vec![2, 6],
          vec![3, 4],
          vec![6, 4],
        ],
        true,
      ),
      (
        -1,
        vec![
          vec![1, 2],
          vec![0, 3],
          vec![0, 4],
          vec![1, 5],
          vec![2, 6],
          vec![3, 4],
          vec![6, 4],
        ],
        false,
      ),
    ];

    for (node, graph, expected) in test_cases {
      let actual = breadth_first_search(node, &graph);

      assert_eq!(expected, actual);
    }
  }
}
