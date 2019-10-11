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

auto main() -> int
{
  auto [codeTable, encodedString] = Huffman::encode("hello world!");
  std::cout << encodedString << '\n';

  std::cout << Huffman::decode(codeTable, encodedString) << '\n';
}