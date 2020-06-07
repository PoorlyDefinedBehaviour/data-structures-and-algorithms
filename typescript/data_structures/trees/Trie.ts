class Node {
  public terminal: boolean = false;
  public children: Map<string, Node> = new Map<string, Node>();
}

export default class Trie {
  private _root = new Node();

  public root = (): Node => this._root;

  public insert(word: string, node: Node = this._root): this {
    const letter = word.charAt(0);

    if (!node.children.has(letter)) {
      node.children.set(letter, new Node());
    }

    if (word.length === 1) {
      node.children.get(letter)!.terminal = true;
    } else {
      this.insert(word.slice(1), node.children.get(letter));
    }

    return this;
  }

  public search(word: string, node: Node = this._root): boolean {
    if (word.length === 0) {
      return node.terminal;
    }

    const letter = word.charAt(0);

    return node.children.has(letter)
      ? this.search(word.slice(1), node.children.get(letter))
      : false;
  }

  public print(node: Node = this._root): this {
    for (const [letter, child] of node.children.entries()) {
      console.log(letter);
      this.print(child);
    }
    return this;
  }
}
