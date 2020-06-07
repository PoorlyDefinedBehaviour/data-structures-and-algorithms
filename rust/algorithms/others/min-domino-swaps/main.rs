fn min(numbers: &Vec<i32>) -> i32 {
  let mut min = numbers[0];

  for number in numbers.iter() {
    if *number < min {
      min = *number;
    }
  }

  min
}

fn min_swaps(target: i32, a: &Vec<i32>, b: &Vec<i32>) -> i32 {
  let mut swaps = 0;

  for i in 0..a.len() {
    if a[i] != target && b[i] != target {
      return std::i32::MAX;
    }

    if a[i] != target && b[i] == target {
      swaps += 1;
    }
  }

  swaps
}

fn min_domino_rotations(a: Vec<i32>, b: Vec<i32>) -> i32 {
  let swaps = min(&vec![
    min_swaps(a[0], &a, &b),
    min_swaps(b[0], &a, &b),
    min_swaps(a[0], &b, &a),
    min_swaps(b[0], &b, &a),
  ]);

  match swaps {
    std::i32::MAX => -1,
    _ => swaps,
  }
}

fn main() {
  /*
       In a row of dominoes, A[i] and B[i] represent the top and bottom halves of the i-th domino.
        (A domino is a tile with two numbers from 1 to 6 - one on each half of the tile.)

       We may rotate the i-th domino, so that A[i] and B[i] swap values.

       Return the minimum number of rotations so that all the values in A are the same,
       or all the values in B are the same.

       If it cannot be done, return -1.

       Example 1:

       Input: A = [2,1,2,4,2,2], B = [5,2,6,2,3,2]
       Output: 2

       Example 2:

       Input: A = [3,5,1,2,3], B = [3,6,3,3,4]
       Output: -1
       Explanation:
       In this case, it is not possible to rotate the dominoes to make one row of values equal.

       Note:

           1 <= A[i], B[i] <= 6
           2 <= A.length == B.length <= 20000
  */
  println!(
    "{:?}",
    min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2])
  );
  println!(
    "{:?}",
    min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4])
  )
}
