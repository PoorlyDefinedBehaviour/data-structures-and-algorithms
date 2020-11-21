const tree = {
  value: 0,
  right: {
    value: 5,
    right: { value: 6, left: null, right: null },
    left: { value: 4, left: null, right: null },
  },
  left: {
    value: -5,
    left: { value: -6, left: null, right: null },
    right: { value: -4, left: null, right: null },
  },
};

const inverse = (node) => {
  if (!node) {
    return;
  }

  // eslint-disable-next-line no-param-reassign
  [node.left, node.right] = [node.right, node.left];

  inverse(node.left);
  inverse(node.right);
};

console.log("before", JSON.stringify(tree, null, 2));

inverse(tree);

console.log("after", JSON.stringify(tree, null, 2));
