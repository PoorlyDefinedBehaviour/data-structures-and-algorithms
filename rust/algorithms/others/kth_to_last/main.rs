//! Return Kth to Last: Implement an algorithm to find the kth to last element of a singly linked list.

use std::collections::LinkedList;

/// time O(n)
/// space O(1)
fn kth_to_last<T>(list: &LinkedList<T>, k: usize) -> Option<&T> {
    if k > list.len() {
        return None;
    }

    let mut back = list.iter();
    let mut front = list.iter();

    for _ in 0..k {
        let _ = front.next();
    }

    let mut next = None;

    while front.next().is_some() {
        next = back.next();
    }

    next
}

/// time O(n)
/// space O(1)
fn kth_to_last_2<T>(list: &LinkedList<T>, k: usize) -> Option<&T> {
    if k == 0 {
        return list.back();
    }

    let mut iter = list.iter().rev();

    for _ in 0..k {
        iter.next();
    }

    iter.next()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let list: LinkedList<i32> = [1, 2, 3, 4, 5].into_iter().collect();
        assert_eq!(Some(&5), kth_to_last(&list, 0));
        assert_eq!(Some(&3), kth_to_last(&list, 2));
        assert_eq!(Some(&1), kth_to_last(&list, 4));

        assert_eq!(Some(&5), kth_to_last_2(&list, 0));
        assert_eq!(Some(&3), kth_to_last_2(&list, 2));
        assert_eq!(Some(&1), kth_to_last_2(&list, 4));
    }
}
