import BinarySearchTree from "./trees/BinarySearchTree";

function main(): void {
  const tree = new BinarySearchTree<number>();
  tree
    .insert(1)
    .insert(2)
    .insert(3);

  tree.remove(1);
  console.log(`tree.height() => ${tree.height()}`);

  console.log(tree.find(1));
  console.log(tree.find(2));
  console.log(tree.find(3));
}
main();
