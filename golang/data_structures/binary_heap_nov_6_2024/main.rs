#[derive(Debug)]
struct Heap<T> {
    nodes: Vec<Option<T>>,
}

impl<T: std::fmt::Debug + std::cmp::PartialOrd> Heap<T> {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    fn push(&mut self, v: T) {
        let mut new_node_index = self.nodes.len();

        self.nodes.push(Some(v));

        let mut parent_index = parent(new_node_index);

        while let Some(index) = parent_index {
            if self.nodes[new_node_index] <= self.nodes[index] {
                break;
            }

            let tmp = self.nodes[index].take();
            self.nodes[index] = self.nodes[new_node_index].take();
            self.nodes[new_node_index] = tmp;
            parent_index = parent(index);
            new_node_index = index;
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.nodes.is_empty() {
            return None;
        }

        let nodes_len = self.nodes.len();

        let value = self.nodes[0].take();
        // TODO: reduce vec size.
        self.nodes[0] = self.nodes[nodes_len - 1].take();

        let mut current_index = 0;
        loop {
            let left = left_child(current_index);
            let right = right_child(current_index);

            if self.nodes.get(left) >= self.nodes.get(right)
                && self.nodes.get(left).is_some()
                && self
                    .nodes
                    .get(left)
                    .map(|v| v > &self.nodes[current_index])
                    .unwrap_or(false)
            {
                let tmp = self.nodes[current_index].take();
                self.nodes[current_index] = self.nodes[left].take();
                self.nodes[left] = tmp;
                current_index = left;
            } else if self.nodes.get(right).is_some()
                && self
                    .nodes
                    .get(right)
                    .map(|v| v > &self.nodes[current_index])
                    .unwrap_or(false)
            {
                let tmp = self.nodes[current_index].take();
                self.nodes[current_index] = self.nodes[right].take();
                self.nodes[right] = tmp;
                current_index = right
            } else {
                break;
            }
        }

        self.nodes.truncate(nodes_len - 1);

        value
    }
}

fn parent(i: usize) -> Option<usize> {
    if i == 0 {
        None
    } else {
        Some((i - 1) / 2)
    }
}

fn left_child(i: usize) -> usize {
    2 * i + 1
}

fn right_child(i: usize) -> usize {
    2 * i + 2
}

fn main() {
    let mut heap = Heap::new();
    heap.push(1);
    heap.push(2);
    heap.push(3);
    heap.push(5);
    heap.push(4);
    dbg!(&heap);

    dbg!(heap.pop());
    dbg!(heap.pop());
    dbg!(heap.pop());
    dbg!(heap.pop());
    dbg!(heap.pop());
    dbg!(heap.pop());
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    use super::*;
    use quickcheck::quickcheck;

    #[derive(Debug, Clone)]
    enum Action {
        Push(i32),
        Pop,
    }

    impl quickcheck::Arbitrary for Action {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            if bool::arbitrary(g) {
                Action::Push(i32::arbitrary(g))
            } else {
                Action::Pop
            }
        }
    }

    quickcheck! {
      #[test]
      fn test_heap(xs: Vec<i32>) -> bool{
        let mut heap = Heap::new();
        let mut model = BinaryHeap::new();

        for x in xs.iter() {
          heap.push(x);
          model.push(x);
        }

        for _ in 0..xs.len() {
          assert_eq!(model.pop(), heap.pop());
        }

        true
      }

      #[test]
      fn test_heap_2(actions: Vec<Action>) -> bool {
        let mut heap = Heap::new();
        let mut model = BinaryHeap::new();

        for action in actions {
          match action {
            Action::Push(v) => {
              model.push(v);
              heap.push(v);
            },
            Action::Pop => {
              assert_eq!(model.pop(), heap.pop());
            }
          }
        }

        while !model.is_empty() {
          assert_eq!(model.pop(), heap.pop());
        }

        true
      }
    }
}
