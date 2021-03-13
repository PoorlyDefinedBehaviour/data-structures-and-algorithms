use std::collections::HashSet;
use std::collections::LinkedList;

pub type Graph<Representation> = Representation;

pub type AdjencyList = Vec<Vec<i32>>;

/**
 * time O(V + E)
 * space O(2E)
 */
pub fn is_undirected_graph_connected(graph: &Graph<AdjencyList>) -> bool {
  /*    1 - 3 - 5
      /         /
    0         /
      \     /
       2 - 4 - 6

    If every node can be reached starting from any node, the graph is connected
  */

  let mut visited_nodes = HashSet::new();

  let mut stack = LinkedList::new();

  // O(1)
  stack.push_back(0);

  while !stack.is_empty() {
    // O(1)
    let node = stack.pop_back().unwrap();

    // O(1)~*
    visited_nodes.insert(node);

    for neighbor in &graph[node as usize] {
      // O(1)
      if !visited_nodes.contains(neighbor) {
        // O(1)
        stack.push_back(*neighbor);
      }
    }
  }

  visited_nodes.len() == graph.len()
}

fn main() {
  println!("hello world");
}

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
        /*
           1 - 3 - 5
         /         /
        0        /
         \     /
          2 - 4   6
         */
        vec![
          vec![1, 2],
          vec![0, 3],
          vec![0, 4],
          vec![1, 5],
          vec![2],
          vec![3, 4],
          vec![],
        ],
        false,
      ),
      (vec![vec![1], vec![0], vec![3], vec![2]], false),
      (
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
    ];

    for (graph, expected) in test_cases {
      let actual = is_undirected_graph_connected(&graph);

      assert_eq!(expected, actual);
    }
  }
}
