//! Animal Shelter: An animal shelter, which holds only dogs and cats, operates on a strictly"first in, first
//! out" basis.
//!
//! People must adopt either the "oldest" (based on arrival time) of all animals at the shelter,
//! or they can select whether they would prefer a dog or a cat (and will receive the oldest animal of
//! that type). They cannot select which specific animal they would like.
//!
//! Create the data structures to maintain this system and implement operations such as enqueue, dequeueAny, dequeueDog,
//! and dequeueCat. You may use the built-in Linkedlist data structure.

use std::collections::LinkedList;

struct AnimalShelter {
    current_arrival_time: usize,
    dogs: LinkedList<Entry<Dog>>,
    cats: LinkedList<Entry<Cat>>,
}

struct Entry<T> {
    arrival_time: usize,
    animal: T,
}

impl AnimalShelter {
    fn new() -> Self {
        Self {
            current_arrival_time: 0,
            dogs: LinkedList::new(),
            cats: LinkedList::new(),
        }
    }

    fn enqueue(&mut self, animal: Animal) {
        match animal {
            Animal::Cat(cat) => self.cats.push_back(Entry {
                arrival_time: self.current_arrival_time,
                animal: cat,
            }),
            Animal::Dog(dog) => self.dogs.push_back(Entry {
                arrival_time: self.current_arrival_time,
                animal: dog,
            }),
        }

        self.current_arrival_time += 1;
    }

    fn dequeue(&mut self) -> Option<Animal> {
        let dog = self.dogs.front();
        let cat = self.cats.front();

        match (dog, cat) {
            (None, None) => None,
            (Some(_), None) => {
                let entry = self.dogs.pop_front().unwrap();
                Some(Animal::Dog(entry.animal))
            }
            (None, Some(_)) => {
                let entry = self.cats.pop_front().unwrap();
                Some(Animal::Cat(entry.animal))
            }
            (Some(dog_entry), Some(cat_entry)) => {
                if dog_entry.arrival_time < cat_entry.arrival_time {
                    let entry = self.dogs.pop_front().unwrap();
                    Some(Animal::Dog(entry.animal))
                } else {
                    let entry = self.cats.pop_front().unwrap();
                    Some(Animal::Cat(entry.animal))
                }
            }
        }
    }

    fn dequeue_cat(&mut self) -> Option<Cat> {
        self.cats.pop_front().map(|entry| entry.animal)
    }

    fn dequeue_dog(&mut self) -> Option<Dog> {
        self.dogs.pop_front().map(|entry| entry.animal)
    }
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq)]
enum Animal {
    Cat(Cat),
    Dog(Dog),
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq)]
struct Cat {}

impl Cat {
    fn new() -> Self {
        Self {}
    }
}

#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq)]
struct Dog {}

impl Dog {
    fn new() -> Self {
        Self {}
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[derive(Debug, proptest_derive::Arbitrary)]
    enum Action {
        Dequeue,
        DequeueCat,
        DequeueDog,
    }

    proptest! {
      #[test]
      fn simple(action: Action, dogs: Vec<Dog>, cats: Vec<Cat>) {
        let mut shelter = AnimalShelter::new();

        let last_dog = dogs.last().cloned();
        let last_cat = cats.last().cloned();

        for dog in dogs {
          shelter.enqueue(Animal::Dog(dog));
        }

        for cat in cats {
          shelter.enqueue(Animal::Cat(cat));
        }

        match action {
          Action::Dequeue => {
            let dequeued = shelter.dequeue();
            let expected = if let Some(dog) = last_dog {
              Some(Animal::Dog(dog))
            } else if let Some(cat) = last_cat {
              Some(Animal::Cat(cat))
            } else {
              None
            };
            assert_eq!(dequeued, expected);
          }
          Action::DequeueCat => {
            assert_eq!(shelter.dequeue_cat(), last_cat);
          }
          Action::DequeueDog => {
            assert_eq!(shelter.dequeue_dog(), last_dog);
          }
        }
      }
    }
}
