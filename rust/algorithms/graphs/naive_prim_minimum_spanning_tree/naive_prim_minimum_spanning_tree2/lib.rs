type WeightMatrix = Vec<Vec<i32>>;

type EdgeWithCost = (i32, i32, i32);

fn find_minimum_spanning_tree(weight_matrix: &WeightMatrix) -> Vec<EdgeWithCost> {
  /*
    T = ∅;
    U = { 1 };
    while (U ≠ V)
      let (u, v) be the lowest cost edge such that u ∈ U and v ∈ V - U;
      T = T ∪ {(u, v)}
      U = U ∪ {v}
  */
  let num_of_nodes = weight_matrix[0].len();

  let unvisited_nodes: Vec<i32> = (1..num_of_nodes as i32).collect();
  let mut visited_nodes = vec![0];

  let mut visited = vec![false; num_of_nodes];

  let mut minimum_spanning_tree = Vec::new();

  loop {
    let mut visited_node_index: i32 = 0;

    let mut unvisited_node_index: i32 = 0;

    let mut cheapest = std::i32::MAX;

    for visited_node in &visited_nodes {
      for unvisited_node in &unvisited_nodes {
        let i = *visited_node as usize;
        let j = *unvisited_node as usize;
        let cost = weight_matrix[i][j];
        if cost > 0 && cost < cheapest && !visited[j] {
          visited_node_index = *visited_node;
          unvisited_node_index = *unvisited_node;
          cheapest = cost;
        }
      }
    }

    visited_nodes.push(unvisited_nodes[unvisited_node_index as usize - 1]);

    let cheapest_path = (visited_node_index, unvisited_node_index, cheapest);
    visited[unvisited_node_index as usize] = true;
    minimum_spanning_tree.push(cheapest_path);

    if visited_nodes.len() == num_of_nodes {
      return minimum_spanning_tree;
    }
  }
}

fn main() {
  println!(
    "{:?}",
    find_minimum_spanning_tree(&vec![
      vec![0, 2, 0, 6, 0],
      vec![2, 0, 3, 8, 5],
      vec![0, 3, 0, 0, 7],
      vec![6, 8, 0, 0, 9],
      vec![0, 5, 7, 9, 0]
    ])
  );
}
