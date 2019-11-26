package abstracts.NaivePriorityQueue;

import java.util.ArrayList;
import java.util.function.BiPredicate;;

public class NaivePriorityQueue<T> {
  private BiPredicate<T, T> comparator;

  private ArrayList<T> elements = new ArrayList<T>();

  private int find_position_to_insert(final T value){
    for(int i = 0; i <elements.size(); ++i)
      if(comparator.test(value, elements.get(i))) 
        return i;
    
    return elements.size();
  }
  
  public NaivePriorityQueue(final BiPredicate<T, T> comparator){
    this.comparator = comparator;
  }
 
  public int length() { return elements.size(); }
  public boolean is_empty() { return length() == 0; }

  public NaivePriorityQueue<T> push(final T value){
    elements.add(find_position_to_insert(value), value);
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