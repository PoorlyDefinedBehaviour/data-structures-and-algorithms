type Maybe<T> = T | null | undefined

type Node<T> = {
  value: T
  next: Maybe<Node<T>>
}

const makeNode = <T>(value: T): Node<T> => ({ value, next: null })

/**
 * time O(n)
 * space O(1)
 */
function inPlaceReverse<T>(head: Node<T>): Node<T> {
  let currentNode = head.next!

  while (true) {
    const nextNodeNext = currentNode.next!.next
    currentNode.next!.next = currentNode

    const currentNodeNext = currentNode.next
    currentNode.next = nextNodeNext

    const currentHeadNext = head.next
    head.next = currentNodeNext
    head.next!.next = currentHeadNext

    if (!nextNodeNext) {
      break
    }
  }

  const newHead = head.next

  head.next = null

  currentNode.next = head

  return newHead!
}

function main() {
  const head = makeNode(1)
  head.next = makeNode(2)
  head.next.next = makeNode(3)
  head.next.next.next = makeNode(4)
  head.next.next.next.next = makeNode(5)

  const reverseHead = inPlaceReverse(head)

  console.log(JSON.stringify(reverseHead, null, 2))
}
main()
