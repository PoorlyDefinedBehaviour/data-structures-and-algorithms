use std::collections::HashSet;

pub type Graph<Representation> = Representation;

pub type AdjencyList = Vec<Vec<i32>>;

pub fn is_undirected_graph_connected_recursive_bfs(graph: &Graph<AdjencyList>) -> bool {
  /*    1 - 3 - 5
      /         /
    0         /
      \     /
       2 - 4 - 6

    If every node can be reached starting from any node, the graph is connected
  */

  fn go(
    current_node_neighbors: &Vec<i32>,
    graph: &Graph<AdjencyList>,
    visited_nodes: &mut HashSet<i32>,
  ) {
    for neighbor in current_node_neighbors {
      if !visited_nodes.contains(neighbor) {
        visited_nodes.insert(*neighbor);

        go(&graph[*neighbor as usize], &graph, visited_nodes);
      }
    }
  }

  let mut visited_nodes = HashSet::new();

  go(&graph[0], graph, &mut visited_nodes);

  visited_nodes.len() == graph.len()
}

fn main() {
  println!("hello world");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn graph_is_connected_recursive_bfs() {
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
    ];

    for (graph, expected) in test_cases {
      let actual = is_undirected_graph_connected_recursive_bfs(&graph);

      assert_eq!(expected, actual);
    }
  }
}
