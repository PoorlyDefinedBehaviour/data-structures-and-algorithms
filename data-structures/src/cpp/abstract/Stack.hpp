#pragma once

#include "../arrays/DynamicArray.hpp"

template <typename T>
class Stack
{
private:
  DynamicArray<T> elements;

public:
  auto size() -> size_t { return elements.length(); }
  auto is_empty() -> bool { return size() == 0; }

  auto peek() -> T & { return elements.first(); }

  auto push(T const &value) -> void
  {
    elements.push(value);
  }

  auto pop() -> T
  {
    return elements.pop();
  }
};