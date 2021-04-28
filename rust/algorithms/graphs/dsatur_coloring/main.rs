use std::collections::{HashMap, HashSet};

pub fn get_lowest_color_not_in_use_by_neighbors(
  vertex: i32,
  adjacency_list: &HashMap<i32, Vec<i32>>,
  colors: &HashMap<i32, usize>,
) -> usize {
  let neighbors = adjacency_list.get(&vertex).unwrap();

  for color in 0..adjacency_list.len() {
    if neighbors
      .iter()
      .all(|neighbor| colors.get(neighbor) != Some(&color))
    {
      return color;
    }
  }

  adjacency_list.len() + 1
}

pub fn count_colored_neighbors(
  vertex: i32,
  adjacency_list: &HashMap<i32, Vec<i32>>,
  colors: &HashMap<i32, usize>,
) -> usize {
  let neighbors = adjacency_list.get(&vertex).unwrap();

  let unique_colors = neighbors
    .iter()
    .filter_map(|neighbor| colors.get(neighbor))
    .collect::<HashSet<_>>();

  unique_colors.len()
}

pub fn get_uncolored_vertex_with_most_colored_neighbors(
  adjacency_list: &HashMap<i32, Vec<i32>>,
  colors: &HashMap<i32, usize>,
) -> i32 {
  let (vertex_with_most_colored_neighbors, _) = adjacency_list
    .keys()
    .filter(|vertex| colors.get(vertex) == None)
    .map(|vertex| {
      (
        vertex,
        count_colored_neighbors(*vertex, adjacency_list, colors),
      )
    })
    .max_by(|a, b| a.1.cmp(&b.1))
    .unwrap();

  *vertex_with_most_colored_neighbors
}

pub fn saturation_degree_coloring(adjacency_list: &HashMap<i32, Vec<i32>>) -> HashMap<i32, usize> {
  let mut colors = HashMap::new();

  if adjacency_list.is_empty() {
    return colors;
  }

  let starting_vertex = adjacency_list
    .keys()
    .max_by(|a, b| {
      adjacency_list
        .get(a)
        .unwrap()
        .len()
        .cmp(&adjacency_list.get(b).unwrap().len())
    })
    .cloned()
    .unwrap();

  colors.insert(starting_vertex, 0);

  while colors.len() != adjacency_list.len() {
    let vertex = get_uncolored_vertex_with_most_colored_neighbors(adjacency_list, &colors);
    let color = get_lowest_color_not_in_use_by_neighbors(vertex, adjacency_list, &colors);
    colors.insert(vertex, color);
  }

  colors
}

macro_rules! hashmap {
  ($($key: expr => $value: expr), *) => {{
      let mut map = HashMap::new();
      $(map.insert($key, $value);)*
      map
  }};
}

fn main() {
  let graph = hashmap! {
   0 => vec![1, 2, 3, 4],
   1 => vec![0, 2, 4, 5],
   2 => vec![0, 1, 3, 5],
   3 => vec![0, 2, 4],
   4 => vec![0, 1, 3, 5],
   5 => vec![1, 2, 4]
  };

  dbg!(saturation_degree_coloring(&graph));
}
