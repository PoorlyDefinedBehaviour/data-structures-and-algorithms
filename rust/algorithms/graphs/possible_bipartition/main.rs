/*
Given a set of N people (numbered 1, 2, ..., N), we would like to split everyone into two groups of any size.

Each person may dislike some other people, and they should not go into the same group.

Formally, if dislikes[i] = [a, b], it means it is not allowed to put the people numbered a and b into the same group.

Return true if and only if it is possible to split everyone into two groups in this way.

Example 1:

Input: N = 4, dislikes = [[1,2],[1,3],[2,4]]
Output: true
Explanation: group1 [1,4], group2 [2,3]

Example 2:

Input: N = 3, dislikes = [[1,2],[1,3],[2,3]]
Output: false

Example 3:

Input: N = 5, dislikes = [[1,2],[2,3],[3,4],[4,5],[1,5]]
Output: false


Constraints:

    1 <= N <= 2000
    0 <= dislikes.length <= 10000
    dislikes[i].length == 2
    1 <= dislikes[i][j] <= N
    dislikes[i][0] < dislikes[i][1]
    There does not exist i != j for which dislikes[i] == dislikes[j].
*/
use std::collections::{HashMap, HashSet, VecDeque};

/// time O(n)
/// space O(n)
pub fn possible_bipartition_using_sets(dislikes: Vec<Vec<i32>>) -> bool {
  let mut left = HashSet::new();
  let mut right = HashSet::new();

  for dislike in &dislikes {
    let hater = dislike.first().unwrap();
    let hated = dislike.last().unwrap();

    if (left.contains(hater) && left.contains(hated))
      || (right.contains(hater) && right.contains(hated))
    {
      return false;
    }

    if !left.contains(hated) && !right.contains(hater) {
      left.insert(hater);
      right.insert(hated);
    } else {
      right.insert(hater);
      left.insert(hated);
    }
  }

  true
}

pub fn build_adjacency_list_from_dislikes(dislikes: &Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
  let mut adjacency_list = HashMap::new();

  for dislike in dislikes {
    let hater = dislike.first().unwrap().clone();
    let hated = dislike.last().unwrap().clone();

    if !adjacency_list.contains_key(&hater) {
      adjacency_list.insert(hater, vec![]);
    }

    if !adjacency_list.contains_key(&hated) {
      adjacency_list.insert(hated, vec![]);
    }

    adjacency_list.get_mut(&hater).unwrap().push(hated);
    adjacency_list.get_mut(&hated).unwrap().push(hater);
  }

  adjacency_list
}

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

/// time O(Edges + Vertices)
/// space O(n)
pub fn possible_bipartition(dislikes: Vec<Vec<i32>>) -> bool {
  is_bipartite_graph(&build_adjacency_list_from_dislikes(&dislikes))
}

fn main() {
  println!("hello world")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn possible_bipartition_using_sets_tests() {
    let test_cases = vec![
      (vec![vec![1, 2], vec![1, 3], vec![2, 4]], true),
      (vec![vec![1, 2], vec![1, 3], vec![2, 3]], false),
      (
        vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]],
        false,
      ),
    ];

    for (input, expected) in test_cases {
      let actual = possible_bipartition_using_sets(input);
      assert_eq!(actual, expected)
    }
  }

  #[test]
  fn possible_bipartition_tests() {
    let test_cases = vec![
      (vec![vec![1, 2], vec![1, 3], vec![2, 4]], true),
      (vec![vec![1, 2], vec![1, 3], vec![2, 3]], false),
      (
        vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]],
        false,
      ),
    ];

    for (input, expected) in test_cases {
      let actual = possible_bipartition(input);
      assert_eq!(actual, expected)
    }
  }
}
