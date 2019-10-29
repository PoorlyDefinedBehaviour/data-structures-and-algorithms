
import lists.DoubleLinkedList.*;

public class Main {
  public static void main(String[] args) {
    DoubleLinkedList<Integer> list = new DoubleLinkedList<Integer>();

    list.insert(10)
      .insert(20)
      .insert(30)
      .insert(40)
      .insert(50);

    list.insert_at(3, 33);

    System.out.println(String.format("list.length(): %d", list.length()));

    list.remove(10);

    System.out.println(String.format("list.length(): %d", list.length()));

    list.to_stream()
      .map(number -> number * 2)
      .forEach(number -> System.out.println(number));
  }
}