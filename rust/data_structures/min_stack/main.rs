//! Stack Min: How would you design a stack which, in addition to push and pop, has a function min
//! which returns the minimum element? Push, pop and min should all operate in 0(1) time.

use std::{
    collections::VecDeque,
    rc::{Rc, Weak},
};

struct MinStack<T> {
    items: VecDeque<Entry<T>>,
}

struct Entry<T> {
    value: Rc<T>,
    /// The minimum value at the time the entry was inserted
    min_value: Weak<T>,
}

impl<T> MinStack<T>
where
    T: PartialOrd,
{
    fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    // time O(1)
    fn push(&mut self, value: T) {
        let value = Rc::new(value);

        let min_value = match self.items.back() {
            None => Rc::downgrade(&value),
            Some(entry) => {
                if *entry.min_value.upgrade().unwrap() < *value {
                    Weak::clone(&entry.min_value)
                } else {
                    Rc::downgrade(&value)
                }
            }
        };

        self.items.push_back(Entry { value, min_value })
    }

    /// time O(1)
    fn pop(&mut self) -> Option<T> {
        self.items
            .pop_back()
            .map(|entry| match Rc::try_unwrap(entry.value) {
                Ok(value) => value,
                Err(_) => unreachable!("There's only one reference to the entry value"),
            })
    }

    /// time O(1)
    fn min(&self) -> Option<Rc<T>> {
        self.items
            .back()
            .and_then(|entry| entry.min_value.upgrade())
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut stack = MinStack::new();
        stack.push(3);
        assert_eq!(Some(Rc::new(3)), stack.min());
        stack.push(2);
        assert_eq!(Some(Rc::new(2)), stack.min());
        stack.push(1);
        assert_eq!(Some(Rc::new(1)), stack.min());
        let _ = stack.pop();
        assert_eq!(Some(Rc::new(2)), stack.min());
        let _ = stack.pop();
        assert_eq!(Some(Rc::new(3)), stack.min());
        let _ = stack.pop();
        assert_eq!(None, stack.min());
    }
}
