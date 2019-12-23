class Node<T> {
  public left: Node<T>;
  public right: Node<T>;

  constructor(public value: T) {}
}

export type Pair<T, U> = [T, U];
export type Maybe<T> = T | null | undefined;
export type Comparator<T> = (a: T, b: T) => -1 | 0 | 1;
export type Consumer<T> = (a: T) => void;

class DuplicatedElementException extends Error {
  constructor(message: string = "Duplicated element") {
    super(message);
  }
}

class ElementNotFoundException extends Error {
  constructor(value) {
    super(`Element ${value} wasn't found.`);
  }
}

export default class BinarySearchTree<T> {
  private _root: Node<T>;
  private _height: number = 0;

  private comparator: Comparator<T> = (a: T, b: T) => {
    if (a < b) return -1;
    if (a > b) return 1;
    return 0;
  };

  private isLessThan = (a: T, b: T) => this.comparator(a, b) === -1;
  private isGreaterThan = (a: T, b: T) => this.comparator(a, b) === 1;
  private isEqual = (a: T, b: T) => this.comparator(a, b) === 0;

  public setComparator(comparator: Comparator<T>): this {
    this.comparator = comparator;
    return this;
  }

  public root = (): Node<T> => this._root;
  public height = (): number => this._height;
  public isEmpty = (): boolean => this._height === 0;

  public insert(value: T): this {
    if (!this._root) {
      this._root = new Node<T>(value);
      ++this._height;
      return this;
    }

    let currentNode = this._root;
    while (currentNode) {
      if (this.isLessThan(value, currentNode.value)) {
        if (!currentNode.left) {
          currentNode.left = new Node<T>(value);
          break;
        }
        currentNode = currentNode.left;
      } else if (this.isGreaterThan(value, currentNode.value)) {
        if (!currentNode.right) {
          currentNode.right = new Node<T>(value);
          break;
        }
        currentNode = currentNode.right;
      } else {
        throw new DuplicatedElementException();
      }
    }

    ++this._height;
    return this;
  }

  private findNodeAndParent(
    parent: Node<T>,
    node: Node<T>,
    value: T
  ): Pair<Node<T>, Node<T>> {
    if (!node) return [null, null];

    if (this.isLessThan(value, node.value))
      return this.findNodeAndParent(node, node.left, value);
    if (this.isGreaterThan(value, node.value))
      return this.findNodeAndParent(node, node.right, value);

    return [parent, node];
  }

  private findBiggestAndParentFrom(node: Node<T>) {
    let parent: Node<T>;
    let current: Node<T> = node;

    while (current.right) {
      parent = current;
      current = current.right;
    }

    return [parent, current];
  }

  public remove(value: T): this {
    let [nodeToDeleteParent, nodeToDelete] = this.findNodeAndParent(
      null,
      this._root,
      value
    );

    if (!nodeToDeleteParent && !nodeToDelete)
      throw new ElementNotFoundException(value);

    --this._height;

    console.log(nodeToDeleteParent, nodeToDelete);

    if (!nodeToDelete.left) {
      nodeToDelete = nodeToDelete.right;

      if (nodeToDeleteParent) {
        nodeToDeleteParent.right = nodeToDelete;
      } else {
        this._root = nodeToDelete;
      }

      return this;
    }

    const [parent, biggest] = this.findBiggestAndParentFrom(nodeToDelete.left);
    biggest.right = nodeToDelete.right;
    if (parent) {
      parent.right = biggest.right;
      biggest.left = nodeToDelete.left;
    }

    return this;
  }

  public find(value: T): Maybe<Node<T>> {
    let currentNode = this._root;

    while (currentNode) {
      if (this.isEqual(value, currentNode.value)) {
        return currentNode;
      }

      currentNode = this.isLessThan(value, currentNode.value)
        ? currentNode.left
        : currentNode.right;
    }

    return null;
  }

  private preOrderTraversalImpl(node: Node<T>, consumer: Consumer<T>): void {
    if (!node) return;

    consumer(node.value);
    this.preOrderTraversalImpl(node.left, consumer);
    this.preOrderTraversalImpl(node.right, consumer);
  }

  public preOrderTraversal(consumer: Consumer<T>): this {
    this.preOrderTraversalImpl(this._root, consumer);
    return this;
  }

  private postOrderTraversalImpl(node: Node<T>, consumer: Consumer<T>): void {
    if (!node) return;

    this.preOrderTraversalImpl(node.left, consumer);
    this.preOrderTraversalImpl(node.right, consumer);
    consumer(node.value);
  }

  public postOrderTraversal(consumer: Consumer<T>): this {
    this.postOrderTraversalImpl(this._root, consumer);
    return this;
  }
}
