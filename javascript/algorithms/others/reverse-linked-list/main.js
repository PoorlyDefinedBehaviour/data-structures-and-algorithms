const reverse = (head) => {
  let previous = null;
  let current = head;
  let next = null;

  while (current) {
    next = current.next;

    current.next = previous;

    previous = current;

    current = next;
  }

  return previous;
};

const linkedList = {
  value: 1,
  next: {
    value: 2,
    next: {
      value: 3,
      next: {
        value: 4,
        next: {
          value: 5,
          next: null,
        },
      },
    },
  },
};

const newHead = reverse(linkedList);

console.log("reversed list", JSON.stringify(newHead, null, 2));
