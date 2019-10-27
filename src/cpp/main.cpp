#include <iostream>

//#include "../tests/LinkedList.test.hpp"
//#include "../tests/CircularDoubleLinkedList.test.hpp"
//#include "./abstract/Date.hpp"
//#include "./trees/BinarySearchTree.hpp"
//#include "../tests/BinarySearchTree.hpp"
//#include "./hash/HashTable.hpp"
//#include "./lists/DoubleLinkedList.hpp"
//#include "./lists/CircularDoubleLinkedList.hpp"
#include "./trees/HuffmanTree.hpp"
//#include "./trees/AvlTree.hpp"

auto main() -> int
{
  auto [code_table, code] = Huffman::encode("hello world! hello world!");
  std::cout << code << '\n';
}