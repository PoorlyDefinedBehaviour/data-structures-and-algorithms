using System;

namespace lists
{
  class Node<T>
  {
    public T data;
    public Node<T> next = null;

    public Node(T data)
    {
      this.data = data;
    }
  }
  public class LinkedList<T>
  {
    private int size = 0;
    private Node<T> _head = null;

    public int length()
    {
      return size;
    }
    public LinkedList<T> insert(T value)
    {
      if (_head is null)
      {
        _head = new Node<T>(value);
        return this;
      }

      Node<T> current = _head;

      while (current.next != null)
      {
        current = current.next;
      }
      current.next = new Node<T>(value);

      size += 1;
      return this;
    }

    public LinkedList<T> remove_if(Func<T, bool> predicate)
    {
      if (predicate(_head.data))
      {
        _head = _head.next;
        size -= 1;
        return this;
      }

      Node<T> current = _head;

      while (current.next != null)
      {
        if (predicate(current.data))
        {
          current.next = current.next.next;
          size -= 1;
          return this;
        }

        current = current.next;
      }

      return this;
    }

    public T find_if(Func<T, bool> predicate)
    {
      Node<T> current = _head;

      while (current != null)
      {
        if (predicate(current.data))
        {
          return current.data;
        }

        current = current.next;
      }

      return default(T);
    }

    public bool includes(T value)
    {
      return find_if((v) => v.Equals(value)).Equals(value);
    }
  }
}