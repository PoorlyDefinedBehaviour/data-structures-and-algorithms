use std::collections::{HashMap, VecDeque};

/// Graphs that have an odd length cycle aren't bipartite.
/// We can find out if a graph has an odd length cycle using graph coloring.
/// Given any two colors, any vertices adjacent to a vertex V should have a color
/// different than the color that was assigned to V, if this is false for any vertex V,
/// the graph has an odd length cycle and thus it is not bipartite.
pub fn has_odd_length_cycle(adjacency_list: &HashMap<i32, Vec<i32>>) -> bool {
  #[derive(Debug, PartialEq, Clone)]
  enum Colors {
    Red,
    Blue,
    Uncolored,
  }

  let mut vertex_colors = adjacency_list
    .keys()
    .map(|vertex| (vertex, Colors::Uncolored))
    .collect::<HashMap<_, _>>();

  for vertex in adjacency_list.keys() {
    if vertex_colors.get(&vertex) != Some(&Colors::Uncolored) {
      continue;
    }

    let mut queue = VecDeque::new();

    queue.push_back(vertex);

    vertex_colors.insert(vertex, Colors::Red);

    while !queue.is_empty() {
      let current_vertex = queue.pop_front().unwrap();

      let current_vertex_color = vertex_colors.get(current_vertex).cloned().unwrap();
      let current_vertex_opposite_color = if current_vertex_color == Colors::Red {
        Colors::Blue
      } else {
        Colors::Red
      };

      let neighbors = adjacency_list.get(current_vertex).unwrap();

      for neighbor in neighbors {
        let neighbor_color = vertex_colors.get(neighbor).cloned().unwrap();

        if current_vertex_color == neighbor_color {
          return true;
        }

        if neighbor_color == Colors::Uncolored {
          vertex_colors.insert(neighbor, current_vertex_opposite_color.clone());
          queue.push_back(neighbor);
        }
      }
    }
  }

  false
}

/// A graph is considered bipartite if it's vertices can be divided
/// in two sets A and B where all elements of a set are disjoint and
/// all elements in set A are connected to at least one element in set B
pub fn is_bipartite_graph(adjacency_list: &HashMap<i32, Vec<i32>>) -> bool {
  !has_odd_length_cycle(&adjacency_list)
}

fn main() {
  println!("hello world")
}

#[cfg(test)]
mod tests {
  use super::*;

  macro_rules! hashmap {
    ($($key: expr => $value: expr), *) => {{
        let mut map = HashMap::new();
        $(map.insert($key, $value);)*
        map
    }};
  }

  #[test]
  fn is_bipartite_graph_tests() {
    let test_cases = vec![
      (
        hashmap! {
          1 => vec![2,3],
          2 => vec![1, 4],
          3 => vec![1],
          4 => vec![2]
        },
        true,
      ),
      (
        hashmap! {
        1 => vec![2, 3],
        2 => vec![1, 3],
        3 => vec![1, 2]
              },
        false,
      ),
      (
        hashmap! {
          1 => vec![2, 5],
          2 => vec![1, 3],
          3 => vec![2, 4],
          4 => vec![3, 5],
          5 => vec![4, 1]
        },
        false,
      ),
    ];

    for (input, expected) in test_cases {
      let actual = is_bipartite_graph(&input);
      assert_eq!(actual, expected)
    }
  }
}
