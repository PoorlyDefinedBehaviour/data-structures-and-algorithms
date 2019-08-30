#include <iostream>

//#include "../tests/LinkedList.test.hpp"
#include "../tests/DoubleLinkedList.test.hpp"
//#include "../tests/CircularDoubleLinkedList.test.hpp"

auto main() -> int
{
  //linked_list_test_suite::start();
  doubledlinkedlist_test_suite::start();
  //circulardoublelinkedlist_test_suite::start();

  DoubleLinkedList<int> list;
  list.insert(10);
  list.insert(20);
  list.insert(30);

  std::cout << list;
}