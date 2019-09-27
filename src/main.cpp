#include <iostream>

//#include "../tests/LinkedList.test.hpp"
//#include "../tests/DoubleLinkedList.test.hpp"
//#include "../tests/CircularDoubleLinkedList.test.hpp"
//#include "./abstract/Date.hpp"
//#include "./trees/BinarySearchTree.hpp"
#include "../tests/BinarySearchTree.hpp"
#include <functional>

auto main() -> int
{
  //binary_search_tree_test_suite::start();
  BinarySearchTree<int> tree;
  tree.insert(10)
      .insert(5)
      .insert(25);

  tree.remove(5);

  std::cout << tree;

  std::cout << "tree.size() " << tree.size() << '\n';
}