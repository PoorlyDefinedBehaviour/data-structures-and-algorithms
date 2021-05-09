/// time O(n³)
/// space O(n²)
fn floyd_warshall(adjacency_matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let mut adjacency_matrix = adjacency_matrix.clone();

  let vertices = (0..adjacency_matrix.len()).collect::<Vec<usize>>();

  for k in &vertices {
    for i in &vertices {
      for j in &vertices {
        let k = *k;
        let i = *i;
        let j = *j;

        let cost = std::cmp::min(
          adjacency_matrix[i][j],
          adjacency_matrix[i][k].saturating_add(adjacency_matrix[k][j]),
        );

        adjacency_matrix[i][j] = cost;
      }
    }
  }

  adjacency_matrix
}

fn main() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic_test() {
    let graph = vec![
      vec![0, 3, i32::MAX, 7],
      vec![8, 0, 2, i32::MAX],
      vec![5, i32::MAX, 0, 1],
      vec![2, i32::MAX, i32::MAX, 0],
    ];

    let expected = vec![
      vec![0, 3, 5, 6],
      vec![5, 0, 2, 3],
      vec![3, 6, 0, 1],
      vec![2, 5, 7, 0],
    ];

    let actual = floyd_warshall(&graph);

    assert_eq!(actual, expected);
  }
}
