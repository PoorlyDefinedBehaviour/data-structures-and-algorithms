import BinarySearchTree, { Node } from "./trees/BinarySearchTree";

function breadthFirst<T>(node: Node<T>) {
  const queue = [node];

  while (queue.length > 0) {
    const node = queue.shift()!;

    console.log(node.value);
    if (node.left) queue.push(node.left);
    if (node.right) queue.push(node.right);
  }
}

function main(): void {
  const tree = new BinarySearchTree<string>();
  tree
    .insert("a")
    .insert("b")
    .insert("c")
    .insert("d")
    .insert("e");

  breadthFirst(tree.root());
}
main();
