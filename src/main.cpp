#include <iostream>

//#include "../tests/LinkedList.test.hpp"
//#include "../tests/CircularDoubleLinkedList.test.hpp"
//#include "./abstract/Date.hpp"
//#include "./trees/BinarySearchTree.hpp"
//#include "../tests/BinarySearchTree.hpp"
#include "./hash/HashTable.hpp"
//#include "./lists/DoubleLinkedList.hpp"
//#include "./lists/CircularDoubleLinkedList.hpp"

auto main() -> int
{
  struct S
  {
    int data;
  };

  HashTable<S> table;
  table.set("one", S{1});
  table.set("two", S{2});
  table.set("three", S{3});
  table.set("four", S{4});

  std::cout << table.get("three").data << '\n';
}