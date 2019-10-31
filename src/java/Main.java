import hashs.HashTable.HashTable;

public class Main {
  public static void main(String[] args) {
    HashTable<String, Integer> table = new HashTable<String, Integer>(10);
    System.out.println(String.format("table.size(): %d", table.size()));
    table.set("one", 1);
    System.out.println(String.format("table.size(): %d", table.size()));
  }
}