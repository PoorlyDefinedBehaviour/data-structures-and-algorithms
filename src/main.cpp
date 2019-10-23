#include <iostream>

//#include "../tests/LinkedList.test.hpp"
//#include "../tests/CircularDoubleLinkedList.test.hpp"
//#include "./abstract/Date.hpp"
//#include "./trees/BinarySearchTree.hpp"
//#include "../tests/BinarySearchTree.hpp"
//#include "./hash/HashTable.hpp"
//#include "./lists/DoubleLinkedList.hpp"
//#include "./lists/CircularDoubleLinkedList.hpp"
//#include "./trees/HuffmanTree.hpp"
#include "./trees/AvlTree.hpp"

auto main() -> int
{
  AvlTree<int> tree;
  tree.insert(1)
      .insert(2)
      .insert(89)
      .insert(90)
      .insert(29)
      .insert(3);

  std::cout << tree.find(29)->data << '\n';
}