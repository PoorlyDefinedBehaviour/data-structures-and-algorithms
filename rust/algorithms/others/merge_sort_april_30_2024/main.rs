use std::cmp;

fn merge_sort(xs: &mut [i32]) {
    if xs.is_empty() {
        return;
    }
    sort(xs, 0, xs.len() - 1)
}

fn sort(xs: &mut [i32], start: usize, end: usize) {
    if start == end || xs.is_empty() {
        return;
    }

    let middle = (start + end) / 2;

    sort(xs, start, middle);

    sort(xs, middle + 1, end);

    merge(xs, start, middle, end);
}

fn merge(xs: &mut [i32], start: usize, middle: usize, end: usize) {
    let mut sorted = Vec::with_capacity(end - start);
    let mut i = start;
    let mut j = middle + 1;

    while i <= middle || j <= end {
        if i <= middle && j <= end {
            match xs[i].cmp(&xs[j]) {
                cmp::Ordering::Less | cmp::Ordering::Equal => {
                    sorted.push(xs[i]);
                    i += 1;
                }
                cmp::Ordering::Greater => {
                    sorted.push(xs[j]);
                    j += 1;
                }
            }
        } else if i <= middle {
            sorted.push(xs[i]);
            i += 1;
        } else {
            sorted.push(xs[j]);
            j += 1;
        }
    }

    for (i, x) in (start..=end).zip(sorted) {
        xs[i] = x;
    }
}

fn main() {
    let mut xs = [3, 2, 1, 4];
    merge_sort(&mut xs);
    dbg!(&xs);
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
      #[test]
      fn test_merge_sort(mut xs: Vec<i32>) {
          merge_sort(&mut xs);

          for (previous, current) in (0..xs.len()).zip(1..xs.len()) {
            assert!(xs[previous] <=xs[current]);
          }
      }
    }
}
