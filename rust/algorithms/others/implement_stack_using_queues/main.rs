// 225. Implement Stack using Queues - https://leetcode.com/problems/implement-stack-using-queues/
//
// Implement a last-in-first-out (LIFO) stack using only two queues.
// The implemented stack should support all the functions of a normal stack (push, top, pop, and empty).
//
// Implement the Stack class:
//
//     void push(int x) Pushes element x to the top of the stack.
//     int pop() Removes the element on the top of the stack and returns it.
//     int top() Returns the element on the top of the stack.
//     boolean empty() Returns true if the stack is empty, false otherwise.
//
// Notes:
//
//     You must use only standard operations of a queue, which means that only push to back, peek/pop from front, size and is empty operations are valid.
// Depending on your language, the queue may not be supported natively. You may simulate a queue using a list or deque (double-ended queue) as long as you use only a queue's standard operations.
struct Stack<T> {
    queue: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    // time O(1)
    pub fn push(&mut self, value: T) {
        self.queue.push(value);
    }

    // time O(n)
    pub fn pop(&mut self) -> Option<T> {
        if self.queue.is_empty() {
            return None;
        }

        let value = self.queue.pop();

        for _ in 0..self.queue.len() {
            let value = self.queue.pop().unwrap();
            self.queue.push(value);
        }

        value
    }

    // time O(1)
    pub fn top(&self) -> Option<&T> {
        self.queue.last()
    }

    // time O(1)
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

fn main() {
    let mut stack = Stack::new();

    stack.push(1);
    stack.push(2);

    dbg!(stack.top());
    dbg!(stack.pop());
    dbg!(stack.top());
    dbg!(stack.pop());
    dbg!(stack.top());
    dbg!(stack.pop());
}
