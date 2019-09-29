#pragma once

#include "../utils/Quicksort.hpp"
#include "../utils/Print.hpp"

template <typename T, size_t size>
class StaticArray
{
private:
  T data[size];

public:
  auto length() const -> size_t { return size; }
  auto is_empty() const -> bool { return length() == 0; }

  auto operator[](size_t index) -> T & { return data[index]; }

  auto front() -> T & { return data[0]; }
  auto back() -> T & { return data[size - 1]; }

  auto raw() -> T * { return data; }

  auto fill(T const &value) -> void
  {
    for (size_t i = 0; i < size; ++i)
      data[i] = value;
  }

  auto includes(const T &element) -> bool
  {
    for (size_t i = 0; i < size; ++i)
      if (element == data[i])
        return true;
    return false;
  }

  auto index_of(const T &element) -> size_t
  {
    for (size_t i = 0; i < size; ++i)
      if (element == data[i])
        return i;
    return -1;
  }

  template <typename lambda>
  auto find(const lambda &fn) -> size_t
  {
    for (size_t i = 0; i < size; ++i)
      if (fn(data[i]))
        return i;

    return -1;
  }

  template <typename lambda>
  auto map(const lambda &fn) -> StaticArray<T, size>
  {
    StaticArray<T, size> array;

    for (size_t i = 0; i < size; ++i)
      array[i] = (fn(data[i]));

    return array;
  }

  template <typename lambda>
  auto filter(const lambda &fn) -> StaticArray<T, size>
  {
    StaticArray<T, size> array;

    for (size_t i = 0; i < size; ++i)
      if (fn(data[i]))
        array[i] = (data[i]);

    return array;
  }

  template <typename lambda>
  auto reduce(const lambda &fn) -> T
  {
    T accum;

    for (size_t i = 0; i < size; ++i)
      accum = fn(accum, data[i]);

    return accum;
  }

  template <typename lambda>
  auto sort(const lambda &fn) -> void
  {
    utils::quicksort(data, 0, size - 1, fn);
  }

  auto print() -> void
  {
    for (size_t i = 0; i < size; ++i)
      utils::print(data[i]);
  }
};