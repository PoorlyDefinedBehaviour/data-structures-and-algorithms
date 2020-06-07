class Node {
  public mother: Node;
  public father: Node;

  constructor(public readonly person_name: string) {}
}

export type Predicate<T> = (x: T) => boolean;

export default class FamilyTree {
  private _root: Node;

  public root = (): Node => this._root;

  private insert_impl(
    node: Node,
    person_name: string,
    type: string,
    parent_name: string
  ): void {
    if (!node) {
      return;
    }

    if (node.person_name === person_name) {
      if (type === "mother") {
        node.mother = new Node(parent_name);
      } else {
        node.father = new Node(parent_name);
      }
    } else {
      this.insert_impl(node.mother, person_name, type, parent_name);
      this.insert_impl(node.father, person_name, type, parent_name);
    }
  }

  public insert(
    person_name: string,
    type: string,
    parent_name: string
  ): FamilyTree {
    if (!this._root) {
      this._root = new Node(person_name);
    } else {
      this.insert_impl(this._root, person_name, type, parent_name);
    }

    return this;
  }

  private remove_impl(
    previous_node: Node,
    current_node: Node,
    person_name: string
  ): void {
    if (!current_node) {
      return;
    }

    if (current_node.person_name === person_name) {
      if (!previous_node) {
        this._root = null;
      } else if (
        previous_node.mother &&
        previous_node.mother.person_name === person_name
      ) {
        previous_node.mother = null;
      } else {
        previous_node.father = null;
      }
    } else {
      this.remove_impl(current_node, current_node.mother, person_name);
      this.remove_impl(current_node, current_node.father, person_name);
    }
  }

  public remove(person_name: string): FamilyTree {
    this.remove_impl(null, this._root, person_name);
    return this;
  }

  private find_impl(node: Node, predicate: Predicate<Node>): Node {
    if (!node) {
      return null;
    }

    if (predicate(node)) {
      return node;
    }

    return (
      this.find_impl(node.mother, predicate) ||
      this.find_impl(node.father, predicate)
    );
  }

  public find = (predicate: Predicate<Node>): Node =>
    this.find_impl(this._root, predicate);

  public contains = (predicate: Predicate<Node>): boolean =>
    !!this.find(predicate);
}
