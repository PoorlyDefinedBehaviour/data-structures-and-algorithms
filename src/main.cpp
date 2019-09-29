#include <iostream>

//#include "../tests/LinkedList.test.hpp"
//#include "../tests/DoubleLinkedList.test.hpp"
//#include "../tests/CircularDoubleLinkedList.test.hpp"
//#include "./abstract/Date.hpp"
//#include "./trees/BinarySearchTree.hpp"
//#include "../tests/BinarySearchTree.hpp"
#include "./hash/HashTable.hpp"

auto main() -> int
{
  HashTable<int> hash_table;
  hash_table.set("test", 10);
  hash_table.set("test1", 20);
  std::cout << "hash_table.has('test') " << std::boolalpha << hash_table.has("test") << '\n';
  std::cout << "hash_table.has('test1') " << std::boolalpha << hash_table.has("test1") << '\n';

  hash_table.remove("test");
  std::cout << "hash_table.has('test') " << std::boolalpha << hash_table.has("test") << '\n';
  std::cout << "hash_table.has('test1') " << std::boolalpha << hash_table.has("test1") << '\n';
}