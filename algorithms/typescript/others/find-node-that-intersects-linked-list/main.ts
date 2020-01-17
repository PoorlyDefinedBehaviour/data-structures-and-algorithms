class LinkedListNode {
  public length: number = 0;
  public next: LinkedListNode;
  constructor(public readonly value) {}
}

type Maybe<T> = T | undefined | null;

function findIntersectingNode(
  nodeOne: LinkedListNode,
  nodeTwo: LinkedListNode
): Maybe<LinkedListNode> {
  const lengthDifference = Math.abs(nodeTwo.length - nodeTwo.length);

  for (let i = 0; i < lengthDifference; ++i) {
    nodeOne = nodeOne.next;
  }

  while (nodeOne && nodeTwo) {
    if (nodeOne === nodeTwo) {
      return nodeOne;
    }

    nodeOne = nodeOne.next;
    nodeTwo = nodeTwo.next;
  }

  return null;
}

function main() {
  /* 
  Given two linked lists, find the node where an intersection happens, if there is an intersection.

  Example:
    List A =>  1 ->  2 ->  3 -> 4 -> 5
    List B => -1 -> -2 -> -3 -> 4

    Intersection happens on node 4
*/
  const headOne = new LinkedListNode(1);
  headOne.next = new LinkedListNode(2);
  headOne.next.next = new LinkedListNode(3);
  headOne.next.next.next = new LinkedListNode(4);
  headOne.next.next.next.next = new LinkedListNode(5);
  headOne.length = 5;

  const headTwo = new LinkedListNode(-1);
  headTwo.next = new LinkedListNode(-2);
  headTwo.next.next = new LinkedListNode(-3);
  headTwo.next.next.next = headOne.next.next.next;
  headTwo.length = 4;

  console.log(findIntersectingNode(headOne, headTwo));
}
main();
