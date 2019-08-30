#include <cassert>

#include "../src/lists/CircularDoubleLinkedList.hpp"

namespace circulardoublelinkedlist_test_suite
{
static auto length() -> void
{
  CircularDoubleLinkedList<int> list;
  assert(list.length() == 0);
}

static auto is_empty() -> void
{
  CircularDoubleLinkedList<int> list;
  assert(list.is_empty());
  list.insert(1);
  assert(!list.is_empty());
}

static auto head() -> void
{
  CircularDoubleLinkedList<int> list;
  assert(list.head() == nullptr);
  list.insert(1);
  list.insert(2);
  list.insert(3);
  assert(list.head()->get_data() == 1);
}

static auto insert_at() -> void
{
  CircularDoubleLinkedList<int> list;
  list.insert_at(0, 1);
  list.insert_at(1, 2);
  list.insert_at(2, 3);
  assert(list.head()->get_data() == 1 &&
         list.tail()->get_data() == 3 &&
         list.length() == 3);
}

static auto insert() -> void
{
  CircularDoubleLinkedList<int> list;
  list.insert(10);
  list.insert(20);
  list.insert(30);

  assert(list.head()->get_data() == 10 &&
         list.tail()->get_data() == 30 &&
         list.length() == 3);
}

static auto remove_at() -> void
{
  CircularDoubleLinkedList<int> list;
  list.insert(10);
  list.insert(20);
  list.insert(30);

  //assert(list.length() == 3 &&
  //     list.at(1)->get_data() == 20);

  list.remove_at(1);

  //assert(list.length() == 2 &&
  //     list.at(1)->get_data() == 30);
}

static auto remove() -> void
{
  CircularDoubleLinkedList<int> list;
  list.insert(10);
  list.insert(20);
  list.insert(30);

  assert(list.length() == 3 && list.tail()->get_data() == 30);

  list.remove(30);

  assert(list.length() == 2 && list.tail()->get_data() == 20);
}

static auto pop() -> void
{
  CircularDoubleLinkedList<int> list;
  list.insert(10);
  list.insert(20);
  list.insert(30);

  assert(list.pop()->get_data() == 30 && list.length() == 2);
}

static auto find() -> void
{
  CircularDoubleLinkedList<int> list;
  list.insert(10);
  list.insert(20);
  list.insert(30);

  assert(list.find(30)->get_data() == 30 &&
         list.find(40) == nullptr);
}

static auto at() -> void
{
  CircularDoubleLinkedList<int> list;
  list.insert(10);
  list.insert(20);
  list.insert(30);

  assert(list.at(1)->get_data() == 20);

  list.remove(20);

  assert(list.at(1)->get_data() == 30);
}

static auto includes() -> void
{
  CircularDoubleLinkedList<int> list;
  list.insert(10);
  list.insert(20);
  list.insert(30);

  assert(list.includes(30) && !list.includes(40));
}

static auto tail() -> void
{
  CircularDoubleLinkedList<int> list;
  assert(list.tail() == nullptr);
  list.insert(10);
  assert(list.tail()->get_data() == 10);
}

static auto start() -> void
{
  length();
  is_empty();
  head();
  insert_at();
  insert();
  remove_at();
  remove();
  pop();
  find();
  at();
  includes();
  tail();
}
} // namespace circulardoublelinkedlist_test_suite