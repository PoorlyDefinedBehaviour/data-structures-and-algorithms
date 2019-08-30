#pragma once

#include <stdexcept>

#include "../utils/Swap.hpp"
#include "../arrays/DynamicArray.hpp"

template <typename T>
class MinHeap
{
private:
  DynamicArray<T> data;
  size_t size = 0;

  auto parent(size_t index) const -> size_t { return (index - 1) / 2; }
  auto left(size_t index) const -> size_t { return (2 * index + 1); }
  auto right(size_t index) const -> size_t { return (2 * index + 2); }

  auto decrease(size_t index, const T &value) -> void
  {
    data[index] = value;
    while (index != 0 && data[parent(index)] > data[index])
    {
      utils::swap(data[index], data[parent(index)]);
      index = parent(index);
    }
  }

  auto min() -> T
  {
    if (size == 0)
      throw std::logic_error("Heap has no elements");

    if (size == 1)
    {
      size--;
      return data[0];
    }

    T root = data[0];
    data[0] = data[size - 1];
    size--;
    heapify(0);

    return root;
  }

  auto heapify(size_t index) -> void
  {
    size_t l = left(index);
    size_t r = right(index);
    size_t smallest = index;

    if (l < size && data[l] < data[index])
      smallest = l;
    if (r < size && data[r] < data[smallest])
      smallest = r;
    if (smallest != index)
    {
      utils::swap(data[index], data[smallest]);
      heapify(smallest);
    }
  }

public:
  auto length() const -> size_t { return size; }

  auto push(T const &value) -> void
  {
    data.push(value);
    ++size;

    size_t index = size - 1;

    while (index != 0 && data[parent(index)] > data[index])
    {
      utils::swap(data[index], data[parent(index)]);
      index = parent(index);
    }
  }

  auto remove(size_t index) -> void
  {
    decrease(index, std::numeric_limits<T>::min());
    min();
  }

  auto get_min() -> T & { return data[0]; }

  auto pop_min() -> T { return min(); }

  auto print() -> void
  {
    data.print();
  }
};