#pragma once

#include <algorithm>
#include <stdexcept>
#include <memory>

#include "../utils/RawArrayCopy.hpp"
#include "../utils/Quicksort.hpp"

template <typename T>
class DynamicArray
{
private:
  std::unique_ptr<T[]> data = std::make_unique<T[]>(1);
  size_t size = 0;

public:
  auto length() const -> size_t { return size; }
  auto is_empty() const -> bool { return length() == 0; }
  auto raw() -> T * { return data.get(); }
  auto first() -> T & { return data[0]; }
  auto last() -> T & { return data[size - 1]; }
  auto first_copy() const -> T { return data[0]; }
  auto last_copy() const -> T { return data[size - 1]; }

  auto operator[](size_t index) -> T &
  {
    if (index > size - 1)
      throw std::out_of_range("Invalid index");

    return data[index];
  }

  auto shift() -> T
  {
    if (size == 0)
      throw std::logic_error("Array is empty");

    T value = data[0];

    --size;
    if (size == 0)
      return value;

    auto temp = std::make_unique<T[]>(size);

    utils::raw_array_copy(data.get(), temp.get(), 1, size + 1);

    data.swap(temp);

    return value;
  }

  auto pop() -> T
  {
    if (size == 0)
      throw std::logic_error("Array is empty");

    --size;
    T value = data[size];

    auto temp = std::make_unique<T[]>(size);
    utils::raw_array_copy(data.get(), temp.get(), 0, size);

    data.swap(temp);

    return value;
  }

  auto push(const T &element) -> void
  {
    if (size == 0)
    {
      data[size++] = element;
      return;
    }

    auto temp = std::make_unique<T[]>(++size);

    utils::raw_array_copy(data.get(), temp.get(), 0, size - 1);

    temp[size - 1] = element;

    data.swap(temp);
  };

  auto remove_at(size_t index) -> void
  {
    if (size == 0)
      throw std::logic_error("Array is empty");

    if (index > size - 1)
      throw std::out_of_range("Invalid index");

    auto temp = std::make_unique<T[]>(size - 1);

    size_t _index = 0;
    for (size_t i = 0; i < size; ++i)
    {
      if (i != index)
      {
        temp[_index] = data[i];
        ++_index;
      }
    }

    --size;
    data.swap(temp);
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
  auto map(const lambda &fn) -> DynamicArray<T>
  {
    DynamicArray<T> array;

    for (size_t i = 0; i < size; ++i)
      array.push(fn(data[i]));

    return array;
  }

  template <typename lambda>
  auto filter(const lambda &fn) -> DynamicArray<T>
  {
    DynamicArray<T> array;

    for (size_t i = 0; i < size; ++i)
      if (fn(data[i]))
        array.push(data[i]);

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