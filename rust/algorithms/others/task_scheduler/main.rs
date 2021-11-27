use std::collections::{BinaryHeap, HashMap, VecDeque};

// 621. Task Scheduler
//
// Given a characters array tasks, representing the tasks a CPU needs to do,
// where each letter represents a different task. Tasks could be done in any order.
// Each task is done in one unit of time. For each unit of time, the CPU could complete either one task or just be idle.
// However, there is a non-negative integer n that represents the cooldown period between two same tasks (the same letter in the array),
// that is that there must be at least n units of time between any two same tasks.
//
// Return the least number of units of times that the CPU will take to finish all the given tasks.
//
// Example 1:
//
// Input: tasks = ["A","A","A","B","B","B"], n = 2
// Output: 8
// Explanation:
// A -> B -> idle -> A -> B -> idle -> A -> B
// There is at least 2 units of time between any two same tasks.
//
// Example 2:
//
// Input: tasks = ["A","A","A","B","B","B"], n = 0
// Output: 6
// Explanation: On this case any permutation of size 6 would work since n = 0.
// ["A","A","A","B","B","B"]
// ["A","B","A","B","A","B"]
// ["B","B","B","A","A","A"]
// ...
// And so on.
//
// Example 3:
//
// Input: tasks = ["A","A","A","A","A","A","B","C","D","E","F","G"], n = 2
// Output: 16
// Explanation:
// One possible solution is
// A -> B -> C -> A -> D -> E -> A -> F -> G -> A -> idle -> idle -> A -> idle -> idle -> A
//
// Constraints:
//
//     1 <= task.length <= 104
//     tasks[i] is upper-case English letter.
//     The integer n is in the range [0, 100].

// time O(n * m)
//    where n is the length of [tasks]
//    and m is [n]
// space O(1) - 26 characters
pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
  if tasks.is_empty() {
    return 0;
  }

  if n == 0 {
    return 1;
  }

  if n == 1 {
    return tasks.len() as i32;
  }

  let mut tasks_frequency = HashMap::new();

  for task in tasks {
    let frequency = tasks_frequency.entry(task).or_insert(0);

    *frequency += 1;
  }

  let mut heap = tasks_frequency
    .into_iter()
    .map(|(_char, count)| count)
    .collect::<BinaryHeap<_>>();

  let mut queue = VecDeque::new();

  let mut now = 0;

  while !heap.is_empty() || !queue.is_empty() {
    if !heap.is_empty() {
      let count = heap.pop().unwrap() - 1;
      if count > 0 {
        queue.push_back((count, now + n));
      }
    }

    if !queue.is_empty() {
      let (_, can_be_executed_after) = queue.get(0).unwrap();
      if *can_be_executed_after == now {
        let (count, _) = queue.pop_front().unwrap();
        heap.push(count);
      }
    }

    now += 1;
  }

  now
}

fn main() {
  dbg!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2));
  dbg!(least_interval(
    vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
    2
  ));
}
