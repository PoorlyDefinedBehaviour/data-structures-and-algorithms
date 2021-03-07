use std::cmp::Ordering;
use std::collections::BinaryHeap;

type WeightMatrix = Vec<Vec<i32>>;

type AdjencyList = Vec<Vec<i32>>;

#[derive(Debug, PartialEq, Eq)]
struct EdgeWithCost {
  from: i32,
  to: i32,
  cost: i32,
}

impl Ord for EdgeWithCost {
  fn cmp(&self, other: &Self) -> Ordering {
    other.cost.cmp(&self.cost)
  }
}

impl PartialOrd for EdgeWithCost {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn add_neighbors_to_priority_queue(
  current_node: i32,
  neighbors: &Vec<i32>,
  weight_matrix: &WeightMatrix,
  visited_nodes: &Vec<bool>,
  priority_queue: &mut BinaryHeap<EdgeWithCost>,
) {
  for neighbor in neighbors {
    let cost = weight_matrix[current_node as usize][*neighbor as usize];

    if visited_nodes[*neighbor as usize] {
      continue;
    }

    priority_queue.push(EdgeWithCost {
      from: current_node,
      to: *neighbor,
      cost,
    })
  }
}

/*
  T = ∅;
  U = { 1 };
  while (U ≠ V)
    let (u, v) be the lowest cost edge such that u ∈ U and v ∈ V - U;
    T = T ∪ {(u, v)}
    U = U ∪ {v}
*/
fn find_minimum_spanning_tree(
  adjency_list: &AdjencyList,
  weight_matrix: &WeightMatrix,
) -> Vec<EdgeWithCost> {
  let mut priority_queue = BinaryHeap::new();

  let mut visited_nodes = vec![false; adjency_list.len()];

  visited_nodes[0] = true;

  add_neighbors_to_priority_queue(
    0,
    &adjency_list[0],
    weight_matrix,
    &visited_nodes,
    &mut priority_queue,
  );

  let mut minimum_spanning_tree = Vec::new();

  let mut visited_edges = 0;

  while visited_edges != adjency_list.len() - 1 && !priority_queue.is_empty() {
    let cheapest_path = priority_queue.pop().unwrap();
    let unvisited_node = cheapest_path.to;

    if visited_nodes[unvisited_node as usize] {
      continue;
    }

    visited_nodes[unvisited_node as usize] = true;

    add_neighbors_to_priority_queue(
      unvisited_node,
      &adjency_list[unvisited_node as usize],
      weight_matrix,
      &visited_nodes,
      &mut priority_queue,
    );

    visited_edges += 1;

    minimum_spanning_tree.push(cheapest_path);
  }

  minimum_spanning_tree
}

fn main() {
  let adjency_list = vec![
    vec![1, 3],
    vec![0, 2, 3, 4],
    vec![1, 4],
    vec![0, 1, 4],
    vec![1, 2, 3],
  ];

  let weight_matrix = vec![
    vec![0, 2, 0, 6, 0],
    vec![2, 0, 3, 8, 5],
    vec![0, 3, 0, 0, 7],
    vec![6, 8, 0, 0, 9],
    vec![0, 5, 7, 9, 0],
  ];

  let expected = vec![
    EdgeWithCost {
      from: 0,
      to: 1,
      cost: 2,
    },
    EdgeWithCost {
      from: 1,
      to: 2,
      cost: 3,
    },
    EdgeWithCost {
      from: 1,
      to: 4,
      cost: 5,
    },
    EdgeWithCost {
      from: 0,
      to: 3,
      cost: 6,
    },
  ];

  let actual = find_minimum_spanning_tree(&adjency_list, &weight_matrix);

  assert_eq!(expected, actual);
}
