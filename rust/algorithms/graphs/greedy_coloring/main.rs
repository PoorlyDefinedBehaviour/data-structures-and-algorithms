use std::collections::HashMap;

fn get_lowest_color_number_unused_by_neighbors(
  available_colors: &[usize],
  vertex_colors: &HashMap<i32, usize>,
  neighbors: &[i32],
) -> usize {
  available_colors
    .iter()
    .find(|color| {
      neighbors
        .iter()
        .all(|neighbor| vertex_colors.get(neighbor) != Some(color))
    })
    .cloned()
    .unwrap()
}

/// Gives each vertex in the graph a color. The color given to a vertex is the
/// first color in the list that's not being used by any of it's neighbors
pub fn greedy_color(adjacency_list: &HashMap<i32, Vec<i32>>) -> HashMap<i32, usize> {
  let available_colors = (0..adjacency_list.keys().len()).collect::<Vec<_>>();
  let vertices = {
    let mut vertices = adjacency_list.keys().collect::<Vec<_>>();
    vertices.sort();
    vertices
  };

  let mut vertex_colors = HashMap::new();

  for vertex in vertices {
    let color = get_lowest_color_number_unused_by_neighbors(
      &available_colors,
      &vertex_colors,
      adjacency_list.get(vertex).unwrap(),
    );

    vertex_colors.insert(*vertex, color);
  }

  vertex_colors
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
  fn greey_coloring_tests() {
    let test_cases = vec![
      (HashMap::new(), HashMap::new()),
      (
        hashmap! {
          0 => vec![1, 2],
          1 => vec![0, 2, 3, 4],
          2 => vec![0, 1, 3, 4],
          3 => vec![1, 2, 4],
          4 => vec![1, 2, 3]
        },
        hashmap! {
          0 => 0,
          1 => 1,
          2 => 2,
          3 => 0,
          4 => 3
        },
      ),
    ];

    for (graph, color_map) in test_cases {
      let actual = greedy_color(&graph);
      assert_eq!(actual, color_map)
    }
  }
}
