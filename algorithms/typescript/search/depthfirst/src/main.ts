import BinarySearchTree, { Node } from "./trees/BinarySearchTree";

function depthFirstTraversalWithStack<T>(node: Node<T>) {
  const stack = [node];

  while (stack.length > 0) {
    const node = stack.pop()!;

    console.log(node.value);
    if (node.right) stack.push(node.right);
    if (node.left) stack.push(node.left);
  }
}

function depthFirstTraversal<T>(node: Node<T>) {
  if (!node) return;

  console.log(node.value);
  depthFirstTraversal(node.left);
  depthFirstTraversal(node.right);
}

function main(): void {
  const tree = new BinarySearchTree<string>();
  tree
    .insert("a")
    .insert("b")
    .insert("c")
    .insert("d")
    .insert("e");

  depthFirstTraversalWithStack(tree.root());
  depthFirstTraversal(tree.root());
}
main();
