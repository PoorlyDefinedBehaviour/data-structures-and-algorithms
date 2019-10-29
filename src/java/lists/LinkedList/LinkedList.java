package lists.LinkedList;

import java.util.ArrayList;
import java.util.function.Predicate;
import java.util.stream.Stream;

public class LinkedList<T> {
  private class Node {
    public T data;
    public Node next = null;

    public Node(T data) {
      this.data = data;
    }
  }

  private Node _head = null;
  private int _length = 0;

  private void assert_index_is_valid(final int index) {
    if (index < 0 || index > length())
    throw new RuntimeException(String.format("Invalid index at insert_at(%d, T)", index));
  }

  public int length() {
    return _length;
  }

  public LinkedList<T> insert(final T value) {
    return insert_at(_length, value);
  }

  public T find_if(Predicate<T> predicate) {
    Node current = _head;

    while (current != null) {
      if (predicate.test(current.data))
        return current.data;

      current = current.next;
    }

    return null;
  }

  public int find_index(Predicate<T> predicate) {
    Node current = _head;
    int index = 0;

    while (current != null) {
      if (predicate.test(current.data))
        return index;

      current = current.next;
      index += 1;
    }

    return -1;
  }

  public LinkedList<T> remove(final T value) {
    _length -= 1;

    if (value == _head) {
      _head = _head.next;
      return this;
    }

    Node current = _head;
    while (current.next != null) {
      if (current.next == value) {
        current.next = current.next.next;
        return this;
      }
      current = current.next;
    }

    throw new RuntimeException("Object not found at LinkedList.remove()");
  }

  public LinkedList<T> remove_at(final int index) {
    assert_index_is_valid(index);

    _length -= 1;

    if (index == 0) {
      _head = _head.next;
      return this;
    }

    Node current = _head;
    int current_index = 0;
    while (current_index + 1 != index) {
      current = current.next;
      current_index += 1;
    }

    current.next = current.next.next;
    return this;
  }

  public LinkedList<T> insert_at(final int index, final T value) {
    assert_index_is_valid(index);

    _length += 1;

    if (index == 0) {
      Node temp = new Node(value);
      temp.next = _head;
      _head = temp;
      return this;
    }

    Node current = _head;
    int current_index = 0;
    while (current_index + 1 != index) {
      current = current.next;
      current_index += 1;
    }

    Node temp = current.next;
    current.next = new Node(value);
    current.next.next = temp;

    return this;
  }

  public Stream<T> to_stream() {
    ArrayList<T> list = new ArrayList<T>(length());

    Node current = _head;
    while (current != null) {
      list.add(current.data);
      current = current.next;
    }

    return list.stream();
  }
}