//! Delete Middle Node: Implement an algorithm to delete a node in the middle (i.e., any node but
//!   the first and last node, not necessarily the exact middle) of a singly linked list, given only access to
//!   that node.

use std::collections::LinkedList;

/// time O(n)
/// space O(1)
fn remove_node<T>(list: &mut LinkedList<T>, i: usize) {
  // O(n)
  let mut part = list.split_off(i);
  // O(1)
  let _ = part.pop_front();
  // O(1)
  list.append(&mut part);
}

/// time O(n)
/// space O(1)
fn remove_node_2<T>(list: &mut LinkedList<T>, value: &T)
where
  T: PartialEq,
{
  let mut i = 0;

  for element in list.iter() {
    if element == value {
      break;
    }

    i += 1;
  }

  // O(n)
  let mut part = list.split_off(i);
  // O(1)
  let _ = part.pop_front();
  // O(1)
  list.append(&mut part);
}

fn main() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() {
    let mut list: LinkedList<i32> = [1, 2, 3, 4, 5].into_iter().collect();
    let expected: LinkedList<i32> = [1, 2, 4, 5].into_iter().collect();
    remove_node(&mut list, 2);
    assert_eq!(expected, list);

    let mut list: LinkedList<i32> = [1, 2, 3, 4, 5].into_iter().collect();
    let expected: LinkedList<i32> = [1, 2, 4, 5].into_iter().collect();
    remove_node_2(&mut list, &3);
    assert_eq!(expected, list);
  }
}
