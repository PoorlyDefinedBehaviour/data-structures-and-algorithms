/*
 Given two non-empty binary trees s and t, 
 check whether tree t has exactly the same structure and node values with a subtree of s. 
 A subtree of s is a tree consists of a node in s and all of this node's descendants.
  The tree s could also be considered as a subtree of itself.

Example 1:
Given tree s:

     3
    / \
   4   5
  / \
 1   2

Given tree t:

   4 
  / \
 1   2

Return true, because t has the same structure and node values with a subtree of s. 
*/

const isSameTree = (nodeA, nodeB) => {
  if (!nodeA || !nodeB) {
    return nodeA === nodeB;
  }

  if (nodeA.value === nodeB.value) {
    return (
      isSameTree(nodeA.left, nodeB.left) && isSameTree(nodeA.right, nodeB.right)
    );
  }

  return false;
};

const isSubTree = (nodeA, nodeB) => {
  if (!nodeA || !nodeB) {
    return false;
  }

  if (isSameTree(nodeA, nodeB)) {
    return true;
  }

  return isSameTree(nodeA.left, nodeB) || isSameTree(nodeB.left, nodeB);
};

const treeA = {
  value: 3,
  left: {
    value: 4,
    left: { value: 1 },
    right: { value: 2 },
  },
  right: {
    value: 5,
    right: { value: 5 },
  },
};

const treeB = {
  value: 4,
  left: {
    value: 1,
  },
  right: { value: 2 },
};

console.log(isSubTree(treeA, treeB));
