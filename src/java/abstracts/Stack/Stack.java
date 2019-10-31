package abstracts.Stack;

import java.util.ArrayList;

public class Stack<T> {
  private ArrayList<T> elements = new ArrayList<T>();

  public int size() { return elements.size(); }
  public boolean is_empty() { return size() == 0; }

  public Stack<T> push(final T element){
    elements.add(element);
    return this;
  }

  public T peek() { return elements.get(size() - 1); }
  public T pop() {
    final T element = elements.get(size() - 1);
    elements.remove(size() - 1);
    return element;
  }
}