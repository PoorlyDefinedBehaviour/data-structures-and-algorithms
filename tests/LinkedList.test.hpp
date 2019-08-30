#include <cassert>
#include <string>

#include "../src/lists/LinkedList.hpp"

namespace linked_list_test_suite
{
static auto length() -> void
{
  LinkedList<std::string> list;
  assert(list.length() == 0);
}

static auto is_empty() -> void
{
  LinkedList<double> list;
  assert(list.is_empty());
}

static auto insert_at() -> void
{
  LinkedList<int> list;
  list.insert_at(0, 10);
  list.insert_at(1, 20);
  list.insert_at(2, 30);
  assert(list.length() == 3 &&
         list.at(0)->get_data() == 10 &&
         list.at(1)->get_data() == 20 &&
         list.at(2)->get_data() == 30);
}

static auto insert() -> void
{
  LinkedList<float> list;
  list.insert(3.14f);
  list.insert(3.14f);
  list.insert(3.14f);
  assert(list.length() == 3 &&
         list.tail()->get_data() == 3.14f &&
         list.head()->get_data() == 3.14f);
}

static auto remove_at() -> void
{
  struct S
  {
    int data = 10;
  };

  LinkedList<S> list;
  list.insert(S{});
  list.insert(S{});
  list.insert(S{});

  assert(list.length() == 3);

  list.remove_at(1);

  assert(list.length() == 2);

  assert(list.tail()->get_data().data == 10);
}

static auto remove() -> void
{
  LinkedList<int> list;
  list.insert(1);
  list.insert(2);
  list.insert(3);
  list.insert_at(1, 4);
  list.insert_at(2, 5);

  assert(list.length() == 5);

  list.remove(3);

  assert(list.length() == 4);
}

static auto pop() -> void
{
  LinkedList<int> list;
  list.insert(1);
  list.insert(2);
  list.insert(3);

  assert(list.pop()->get_data() == 3 &&
         list.length() == 2);

  assert(list.tail()->get_data() == 2);
}

static auto shift() -> void
{
  LinkedList<int> list;
  list.insert(1);
  list.insert(2);
  list.insert(3);

  assert(list.shift()->get_data() == 1 &&
         list.length() == 2);
}

static auto find() -> void
{
  LinkedList<int> list;
  list.insert(1);
  list.insert(2);
  list.insert(3);

  assert(list.find(3)->get_data() == 3);
  assert(list.find(100) == nullptr);
}

static auto at() -> void
{
  LinkedList<std::string> list;
  list.insert("hello");
  list.insert("world");
  list.insert("!");

  assert(list.at(1)->get_data() == "world");
}

static auto includes() -> void
{
  LinkedList<std::string> list;
  list.insert("hello");
  list.insert("world");
  list.insert("!");

  assert(list.includes("!") && !list.includes("alo"));
}

static auto start() -> void
{
  length();
  is_empty();
  insert_at();
  insert();
  remove_at();
  remove();
  pop();
  shift();
  find();
  at();
  includes();
}
} // namespace linked_list_test_suite