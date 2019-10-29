#include <iostream>
#include <type_traits>
#include <utility>
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

/*
template <typename T>
auto add_two_sfinae(T value,
                    [[maybe_unused]] typename std::enable_if<std::is_floating_point<T>::value>::type *_ = nullptr) -> T
{
  return value + 2;
}

*/

template <typename T>
constexpr auto is_hashable(T value) -> bool
{
  try
  {
    to_string(value);
    return true;
  }
  catch (const std::exception &e)
  {
    return false;
  }
}

template <typename T, [[maybe_unused]] typename std::enable_if<is_hashablevalue>::type *_ = nullptr>
struct S
{
  T data;
  S(T const &data)
      : data(data) {}
};

template <typename... Ts>
auto println(Ts &&... args) -> void { (std::cout << ... << std::forward<Ts>(args...)) << '\n'; }

auto main() -> int
{
  S test("hello world!");
  S s(test);
  println(s.data);
}