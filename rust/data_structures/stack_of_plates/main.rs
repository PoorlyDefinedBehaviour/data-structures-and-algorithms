// Stack of Plates: Imagine a (literal) stack of plates. If the stack gets too high, it might topple.
// Therefore, in real life, we would likely start a new stack when the previous stack exceeds some
// threshold. Implement a data structure SetOfStacks that mimics this. SetOfStacks should be
// composed of several stacks and should create a new stack once the previous one exceeds capacity.
// SetOfStacks.push() and SetOfStacks. pop() should behave identically to a single stack
// (that is, pop() should return the same values as it would if there were just a single stack).

struct StackOfPlates<T> {
    stacks: Vec<Vec<T>>,
    max_stack_size: usize,
}

impl<T> StackOfPlates<T> {
    fn new(max_stack_size: usize) -> Self {
        assert!(max_stack_size > 0);
        Self {
            max_stack_size,
            stacks: Vec::new(),
        }
    }

    fn push(&mut self, value: T) {
        if self.stacks.is_empty() {
            self.stacks.push(vec![]);
        }

        let newest_stack = self.stacks.last_mut().unwrap();
        if newest_stack.len() < self.max_stack_size {
            newest_stack.push(value);
        } else {
            self.stacks.push(vec![value]);
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.stacks.last_mut().and_then(|stack| stack.pop())
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
      #[test]
      fn simple(max_stack_size: usize, values: Vec<i32>) {
        prop_assert!(max_stack_size > 0);

        let mut stack = StackOfPlates::new(max_stack_size);

        let last = values.last().cloned();

        for value in values.into_iter() {
          stack.push(value);
        }

        assert_eq!(last, stack.pop());
      }
    }
}
