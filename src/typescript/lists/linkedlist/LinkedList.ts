class Node<T> {
  public data: T;
  public next: Node<T>;

  constructor(data: T) {
    this.data = data;
  }
}

export default class LinkedList<T> {
  private _head: Node<T>;
  private _length: number = 0;

  public length = (): number => this._length;
  public head = (): Node<T> => this._head;

  private assert_index_is_valid = (index: number): void => {
    if (index < 0 || index > this._length)
      throw new Error(`Invalid index at Linkedlist.insert_at(${index}, T)`);
  };

  public insert = (value: T): LinkedList<T> =>
    this.insert_at(this._length, value);

  public insert_at = (index: number, value: T): LinkedList<T> => {
    this.assert_index_is_valid(index);

    this._length += 1;

    if (index == 0) {
      const temp: Node<T> = new Node(value);
      temp.next = this._head;
      this._head = temp;
      return this;
    }

    let current: Node<T> = this._head;
    let current_index: number = 0;
    while (current_index + 1 != index) {
      current = current.next;
      current_index += 1;
    }

    const temp: Node<T> = current.next;
    current.next = new Node(value);
    current.next.next = temp;

    return this;
  };

  public find_if = (predicate: (value: T) => boolean) => {
    let current: Node<T> = this._head;

    while (current) {
      if (predicate(current.data)) return current.data;

      current = current.next;
    }

    return null;
  };
}
