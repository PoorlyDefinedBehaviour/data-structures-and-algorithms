use std::collections::HashSet;
use std::collections::LinkedList;

pub type Graph<Representation> = Representation;

pub type AdjencyList = Vec<Vec<i32>>;

/**
 * time O(V + E)
 * space O(2E)
 */
pub fn path(starting_node: i32, target_node: i32, graph: &Graph<AdjencyList>) -> Option<Vec<i32>> {
  let mut visited_nodes = HashSet::new();

  let mut queue = LinkedList::new();

  let mut current_path = Vec::new();

  queue.push_back(starting_node);

  while !queue.is_empty() {
    let node = queue.pop_front().unwrap();

    current_path.push(node);

    if node == target_node {
      return Some(current_path);
    }

    visited_nodes.insert(node);

    for neighbor in &graph[node as usize] {
      if !visited_nodes.contains(neighbor) {
        queue.push_back(*neighbor);
      }
    }
  }

  None
}

fn main() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn graph_is_connected_bfs() {
    let test_cases = vec![
      /*
          1 - 3 - 5
        /        /
      0        /
        \    /
          2 - 4 - 6
        */
      (
        0,
        3,
        vec![
          vec![1, 2],
          vec![0, 3],
          vec![0, 4],
          vec![1, 5],
          vec![2, 6],
          vec![3, 4],
          vec![6, 4],
        ],
        Some(vec![0, 1, 2, 3]),
      ),
      (
        0,
        7,
        vec![
          vec![1, 2],
          vec![0, 3],
          vec![0, 4],
          vec![1, 5],
          vec![2, 6],
          vec![3, 4],
          vec![6, 4],
        ],
        None,
      ),
      (
        0,
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
        Some(vec![0, 1, 2, 3, 4, 5]),
      ),
    ];

    for (starting_node, target_node, graph, expected_path) in test_cases {
      let actual = path(starting_node, target_node, &graph);

      assert_eq!(expected_path, actual);
    }
  }
}
