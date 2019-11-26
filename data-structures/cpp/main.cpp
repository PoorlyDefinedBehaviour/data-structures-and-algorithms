#include <iostream>
#include <type_traits>
//#include "../tests/LinkedList.test.hpp"
//#include "../tests/CircularDoubleLinkedList.test.hpp"
//#include "./abstract/Date.hpp"
//#include "./trees/BinarySearchTree.hpp"
//#include "../tests/BinarySearchTree.hpp"
//#include "./hash/HashTable.hpp"
//#include "./lists/DoubleLinkedList.hpp"
//#include "./lists/CircularDoubleLinkedList.hpp"
//#include "./trees/HuffmanTree.hpp"
//#include "./trees/AvlTree.hpp"
#include "./trees/BTree.hpp"

/*
template <typename T>
auto add_two_sfinae(T value,
                    [[maybe_unused]] typename std::enable_if<std::is_floating_point<T>::value>::type *_ = nullptr) -> T
{
  return value + 2;
}

*/

template <typename... Ts>
auto println(Ts const &... args) -> void { (std::cout << ... << args) << '\n'; }

auto main() -> int
{
  println("hello world");
  BTree<int, 2> tree;
  tree.insert(10)
      .insert(20)
      .insert(30)
      .insert(40)
      .insert(50)
      .insert(60)
      .insert(70);

  println("tree.height() ", tree.height());
  tree.print(tree.root());
}