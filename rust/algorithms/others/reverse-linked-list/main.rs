#[derive(Debug, Clone, PartialEq)]
enum List<A: Clone> {
  Cons(A, Box<List<A>>),
  Nil,
}

impl<A: Clone> List<A> {
  pub fn reverse(&self) -> List<A> {
    use List::*;

    fn go<A: Clone>(xs: &List<A>, ys: List<A>) -> List<A> {
      match xs {
        Cons(head, tail) => go(tail, Cons(head.clone(), Box::new(ys))),
        Nil => ys,
      }
    }

    go(self, Nil)
  }
}

fn main() {
  use List::*;

  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  dbg!(list.reverse());
}

#[cfg(test)]
mod tests {
  use super::*;
  use List::*;

  #[test]
  fn reverses_linked_list() {
    let test_cases = vec![
      (
        Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))),
        Cons(3, Box::new(Cons(2, Box::new(Cons(1, Box::new(Nil)))))),
      ),
      (Nil, Nil),
      (Cons(1, Box::new(Nil)), Cons(1, Box::new(Nil))),
    ];

    for (list, expected) in test_cases {
      let actual = list.reverse();

      assert_eq!(actual, expected)
    }
  }
}
