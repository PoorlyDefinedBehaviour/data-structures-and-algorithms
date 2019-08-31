#include <iostream>

//#include "../tests/LinkedList.test.hpp"
//#include "../tests/DoubleLinkedList.test.hpp"
//#include "../tests/CircularDoubleLinkedList.test.hpp"
#include "./abstract/Date.hpp"

auto main() -> int
{
  //linked_list_test_suite::start();
  //doubledlinkedlist_test_suite::start();
  //circulardoublelinkedlist_test_suite::start();
  Date a(1, 1, 1);
  Date b(1, 1, 1);

  std::cout << Date::to_milliseconds(Date::now());
}