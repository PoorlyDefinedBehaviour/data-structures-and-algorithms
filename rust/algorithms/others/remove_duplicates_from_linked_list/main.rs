//! Remove Dups! Write code to remove duplicates from an unsorted linked list.
//!
//! FOLLOW UP
//! How would you solve this problem if a temporary buffer is not allowed?
//! ANSWER: possible in O(n^2)

use std::{
    collections::{HashSet, LinkedList},
    hash::Hash,
};

/// time O(n)
/// space O(n)
fn remove_dups<T>(list: LinkedList<T>) -> LinkedList<T>
where
    T: Clone + Hash + Eq,
{
    let mut seen = HashSet::new();

    let mut result = LinkedList::new();

    for element in list.iter() {
        let value_was_not_in_set = seen.insert(element);
        if value_was_not_in_set {
            result.push_back(element.clone());
        }
    }

    result
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let list: LinkedList<i32> = [1, 1, 2, 3, 3].into_iter().collect();

        let expected: LinkedList<i32> = [1, 2, 3].into_iter().collect();

        assert_eq!(expected, remove_dups(list));
    }
}
