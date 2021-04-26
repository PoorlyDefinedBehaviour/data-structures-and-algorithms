use std::collections::HashMap;

fn is_connected_to_vertex_with_color(
  adjacency_list: &HashMap<i32, Vec<i32>>,
  vertex_colors: &HashMap<i32, usize>,
  current_color: usize,
  vertex: &i32,
) -> bool {
  adjacency_list
    .get(vertex)
    .unwrap()
    .iter()
    .any(|neighbor| vertex_colors.get(neighbor) == Some(&current_color))
}

pub fn welsh_powell_coloring(adjacency_list: &HashMap<i32, Vec<i32>>) -> HashMap<i32, usize> {
  let mut vertex_colors = HashMap::new();

  if adjacency_list.is_empty() {
    return vertex_colors;
  }

  let vertices = {
    let mut vertices = adjacency_list.keys().collect::<Vec<_>>();
    vertices.sort_unstable_by(|a, b| {
      adjacency_list
        .get(b)
        .unwrap()
        .len()
        .cmp(&adjacency_list.get(a).unwrap().len())
    });
    vertices
  };

  let mut current_color = 0;

  for i in 0..vertices.len() {
    let vertex = vertices[i];

    if vertex_colors.contains_key(vertex) {
      continue;
    }

    vertex_colors.insert(*vertex, current_color);

    for j in (i + 1)..vertices.len() {
      let vertex = vertices[j];

      if vertex_colors.contains_key(vertex)
        || is_connected_to_vertex_with_color(adjacency_list, &vertex_colors, current_color, vertex)
      {
        continue;
      }

      vertex_colors.insert(*vertex, current_color);
    }

    current_color += 1;
  }

  vertex_colors
}

fn main() {
  macro_rules! hashmap {
    ($($key: expr => $value: expr), *) => {{
        let mut map = HashMap::new();
        $(map.insert($key, $value);)*
        map
    }};
  }

  let graph = hashmap! {
    0 => vec![1, 7],
    1 => vec![0, 3],
    2 => vec![3],
    3 => vec![2, 1, 8, 10],
    4 => vec![5, 10],
    5 => vec![4, 6],
    6 => vec![5, 10, 7],
    7 => vec![0, 8, 9, 10, 6],
    8 => vec![7, 9, 3],
    9 => vec![8, 7, 10],
    10 => vec![6, 4, 3, 9, 7]
  };

  dbg!(welsh_powell_coloring(&graph));
}
