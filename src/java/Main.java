import lists.LinkedList.*;

public class Main {
  public static void main(String[] args) {
    System.out.println("hello world");
    LinkedList<Integer> list = new LinkedList<Integer>();

    list.insert(10).insert(20).insert(30).insert(40).insert(50);

    System.out.println("list.length() " + list.length());
    System.out.println("list.find_if(number -> number == 30) " + list.find_if(number -> number == 30));
    System.out.println("list.find_index(number -> number == 30) " + list.find_index(number -> number == 30));
  }
}