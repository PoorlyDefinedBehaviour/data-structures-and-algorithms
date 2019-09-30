#pragma once

#include "../arrays/DynamicArray.hpp"

template <typename T>
class Queue
{
private:
  DynamicArray<T> elements;

public:
  auto length() -> size_t { return elements.length(); }
  auto is_empty() -> bool { return length() == 0; }

  auto push(T const &value) -> void
  {
    elements.push(value);
  }

  auto front() -> T & { return elements[0]; }
  auto back() -> T & { return elements[elements.length() - 1]; }

  auto next() -> T { return elements.shift(); }
};