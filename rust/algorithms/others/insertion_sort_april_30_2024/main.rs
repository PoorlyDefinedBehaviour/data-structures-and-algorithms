use std::fmt::Debug;

/// time O(n²)
/// space O(1)
fn insertion_sort<T: PartialOrd + Debug>(xs: &mut [T]) {
    for i in 1..xs.len() {
        let mut key_index = i;
        for j in (0..=(i - 1)).rev() {
            if xs[key_index] < xs[j] {
                xs.swap(key_index, j);
                key_index = j;
            }
        }
    }
}

fn main() {
    let mut xs = [7, 10, 1, 12, 50, 22, 14, 79];
    insertion_sort(&mut xs);
    dbg!(xs);
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
      #[test]
      fn test_insertion_sort(mut xs: Vec<i32>) {
        let mut expected = xs.clone();
        expected.sort();

        insertion_sort(&mut xs);

        assert_eq!(expected, xs);

        let mut i = 0;
        for j in 1..xs.len() {
          assert!(xs[i] < xs[j]);
          i = j;
        }
      }
    }
}
