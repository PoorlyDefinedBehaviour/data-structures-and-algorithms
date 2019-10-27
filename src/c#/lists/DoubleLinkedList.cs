using System;

namespace lists
{
  public class DoubleLinkedList<T>
  {
    private class Node
    {
      public T data;
      public Node previous = null;
      public Node next = null;

      public Node(T data)
      {
        this.data = data;
      }
    }

    private Node _head = null;
    private int size = 0;

    public int length() { return size; }
    public DoubleLinkedList<T> insert(T value)
    {
      size += 1;

      if (_head is null)
      {
        _head = new Node(value);
        return this;
      }

      Node current = _head;
      while (current.next != null)
      {
        current = current.next;
      }

      current.next = new Node(value);
      current.next.previous = current;

      return this;
    }

    public DoubleLinkedList<T> remove_if(Func<T, bool> predicate)
    {
      if (predicate(_head.data))
      {
        _head = _head.next;
        _head.next.previous = _head;
        size -= 1;
        return this;
      }


      Node current = _head;
      while (current.next != null)
      {
        if (predicate(current.next.data))
        {
          if (current.next.next != null)
          {
            current.next = current.next.next;
            current.next.previous = current;
          }
          else
          {
            current.next = null;
          }
          Console.WriteLine("here1");
          size -= 1;
          return this;
        }
        current = current.next;
      }

      return this;
    }

    public T find_if(Func<T, bool> predicate)
    {
      Node current = _head;

      while (current != null)
      {
        if (predicate(current.data))
          return current.data;
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