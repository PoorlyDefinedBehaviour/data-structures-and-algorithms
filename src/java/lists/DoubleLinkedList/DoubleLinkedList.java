package lists.DoubleLinkedList;

import java.util.ArrayList;
import java.util.function.Predicate;
import java.util.stream.Stream;

public class DoubleLinkedList<T> {
  private class Node {
    public T data;
    public Node previous = null;
    public Node next = null;

    public Node(final T data){
      this.data = data;
    }
  }

  private Node _head = null;
  private int _length = 0;

  private void assert_index_is_valid(final int index){
    if(index < 0 || index > length())
      throw new RuntimeException(String.format("Invalid index at insert_at(%d, T)", index));
  }

  public int length() {
    return _length;
  }

  public Node head() {
    return _head;
  }

  public DoubleLinkedList<T> insert(final T value){
    return insert_at(length(), value);
  }

  public DoubleLinkedList<T> remove(final T value){
    _length -= 1;

    if (value.equals(_head.data)) {
      _head = _head.next;
      if(_head.next != null)
        _head.next.previous = _head;
      return this;
    }

    Node current = _head;
    while (current.next != null) {
      if (current.next.data.equals(value)) {
        current.next = current.next.next;
        if(current.next != null)
          current.next.previous = current;
        return this;
      }
      current = current.next;
    }

    throw new RuntimeException("Object not found at LinkedList.remove()");
  }

  public DoubleLinkedList<T> insert_at(final int index, final T value){
    assert_index_is_valid(index);

    _length += 1;

    if(index == 0){
      Node temp = _head;
      _head = new Node(value);
      _head.next = temp;
      if(_head.next != null)
        _head.next.previous = _head;
      return this;
    }

    Node current = _head;
    int current_index = 0;
    while(current_index + 1 != index){
      current = current.next;
      current_index += 1;
    }

    Node temp = current.next;
    current.next = new Node(value);
    current.next.next = temp;
    if(temp != null)
      temp.previous = current.next;

    return this;
  }

  public DoubleLinkedList<T> remove_at(final int index) {
    assert_index_is_valid(index);

    _length -= 1;

    if (index == 0) {
      _head = _head.next;
      if(_head.next != null)
        _head.next.previous = _head;
      return this;
    }

    Node current = _head;
    int current_index = 0;
    while (current_index + 1 != index) {
      current = current.next;
      current_index += 1;
    }

    current.next = current.next.next;
    if(current.next != null)
      current.next.previous = current;
    return this;
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

  public Stream<T> to_stream(){
    ArrayList<T> list = new ArrayList<T>(length());
    
    Node current = _head;
    while(current != null)
    {
      list.add(current.data);
      current = current.next;
    }

    return list.stream();
  }
}