/// 622. Design Circular Queue
///
/// Design your implementation of the circular queue.
/// The circular queue is a linear data structure in which the operations are performed based on FIFO (First In First Out)
/// principle and the last position is connected back to the first position to make a circle. It is also called "Ring Buffer".
///
/// One of the benefits of the circular queue is that we can make use of the spaces in front of the queue.
/// In a normal queue, once the queue becomes full, we cannot insert the next element even if there is a space in front of the queue.
/// But using the circular queue, we can use the space to store new values.
///
/// Implementation the MyCircularQueue class:
///
///     MyCircularQueue(k) Initializes the object with the size of the queue to be k.
///     int Front() Gets the front item from the queue. If the queue is empty, return -1.
///     int Rear() Gets the last item from the queue. If the queue is empty, return -1.
///     boolean enQueue(int value) Inserts an element into the circular queue. Return true if the operation is successful.
///     boolean deQueue() Deletes an element from the circular queue. Return true if the operation is successful.
///     boolean isEmpty() Checks whether the circular queue is empty or not.
///     boolean isFull() Checks whether the circular queue is full or not.
///
/// You must solve the problem without using the built-in queue data structure in your programming language.
///
/// Example 1:
///
/// Input
/// ["MyCircularQueue", "enQueue", "enQueue", "enQueue", "enQueue", "Rear", "isFull", "deQueue", "enQueue", "Rear"]
/// [[3], [1], [2], [3], [4], [], [], [], [4], []]
/// Output
/// [null, true, true, true, false, 3, true, true, true, 4]
///
/// Explanation
/// MyCircularQueue myCircularQueue = new MyCircularQueue(3);
/// myCircularQueue.enQueue(1); // return True
/// myCircularQueue.enQueue(2); // return True
/// myCircularQueue.enQueue(3); // return True
/// myCircularQueue.enQueue(4); // return False
/// myCircularQueue.Rear();     // return 3
/// myCircularQueue.isFull();   // return True
/// myCircularQueue.deQueue();  // return True
/// myCircularQueue.enQueue(4); // return True
/// myCircularQueue.Rear();     // return 4
/// Constraints:
///     1 <= k <= 1000
///     0 <= value <= 1000
///     At most 3000 calls will be made to enQueue, deQueue, Front, Rear, isEmpty, and isFull.

#[derive(Debug)]
struct RingBuffer<T> {
    /// Contains the elements that are in the RingBuffer.
    buffer: Vec<Option<T>>,
    /// Index of the element that's in the front of the queue.
    front: usize,
    /// Index of the last elemented inserted in the RingBuffer.
    /// It goes back to 0 after the RingBuffer is full
    /// so old elements get overwrriten.
    back: usize,
    /// The number of elements the buffer can have.
    max_size: usize,
    /// The number of elements in the buffer.
    size: usize,
}

impl<T> RingBuffer<T> {
    /// Creates a RingBuffer with a maximum size.
    pub fn new(max_size: usize) -> Self {
        let mut buffer = Vec::with_capacity(max_size);

        buffer.resize_with(max_size, || None);

        Self {
            buffer,
            front: 0,
            back: 0,
            max_size,
            size: 0,
        }
    }

    /// time O(1)
    /// space O(1)
    pub fn enqueue(&mut self, value: T) {
        if self.back == self.front {
            self.front += 1;
        }

        if self.front >= self.max_size {
            self.front = 0;
        }

        self.buffer[self.back] = Some(value);

        self.back += 1;

        if self.back >= self.max_size {
            self.back = 0;
        }

        self.size += 1;
    }

    /// time O(1)
    /// space O(1)
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let value = self.buffer[self.front].take();
            self.front += 1;
            self.size -= 1;
            value
        }
    }

    /// time O(1)
    /// space O(1)
    pub fn front(&self) -> Option<&T> {
        self.buffer.get(self.front).unwrap().as_ref()
    }

    /// time O(1)
    /// space O(1)
    pub fn rear(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.buffer.get(self.back - 1).unwrap().as_ref()
        }
    }

    /// time O(1)
    /// space O(1)
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// time O(1)
    /// space O(1)
    pub fn is_full(&self) -> bool {
        self.size == self.max_size
    }
}

fn main() {
    let mut buffer = RingBuffer::new(5);
    buffer.enqueue(1);
    buffer.enqueue(2);
    buffer.enqueue(3);
    buffer.enqueue(4);
    buffer.enqueue(5);
    buffer.enqueue(6);

    dbg!(&buffer);

    dbg!(&buffer.front());

    buffer.enqueue(7);

    dbg!(&buffer.rear());

    dbg!(&buffer);
    dbg!(&buffer.dequeue());
}
