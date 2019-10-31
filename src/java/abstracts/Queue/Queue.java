package abstracts.Queue;

import java.util.ArrayList;

public class Queue<T> {
  private ArrayList<T> elements = new ArrayList<T>();

  public int length() { return elements.size(); }
  public boolean is_empty() { return length() == 0; }

  public Queue<T> push(final T value){
    elements.add(value);
    return this;
  }

  public T front() { return elements.get(0); } 
  public T back() { return elements.get(length() - 1); }

  public T next() { 
    final T element = elements.get(0);
    elements.remove(0);
    return element;
  }
}