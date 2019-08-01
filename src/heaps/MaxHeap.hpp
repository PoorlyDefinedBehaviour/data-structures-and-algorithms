#pragma once

#include <stdexcept>

#include "../utils/Swap.hpp"
#include "../arrays/DynamicArray.hpp"

template <typename T>
class MaxHeap
{
private:
  DynamicArray<T> data;
  size_t size = 0;

  auto parent(size_t index) const -> size_t { return (index - 1) / 2; }
  auto left(size_t index) const -> size_t { return (2 * index + 1); }
  auto right(size_t index) const -> size_t { return (2 * index + 2); }

  auto increase(size_t index, const T &value) -> void
  {
    data[index] = value;
    while (index != 0 && data[parent(index)] < data[index])
    {
      utils::swap(data[index], data[parent(index)]);
      index = parent(index);
    }
  }

  auto max() -> T
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

    if (l < size && data[l] > data[index])
      smallest = l;
    if (r < size && data[r] > data[smallest])
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

    while (index != 0 && data[parent(index)] < data[index])
    {
      utils::swap(data[index], data[parent(index)]);
      index = parent(index);
    }
  }

  auto remove(size_t index) -> void
  {
    increase(index, std::numeric_limits<T>::max());
    max();
  }

  auto get_max() -> T & { return data[0]; }

  auto pop_max() -> T { return max(); }

  auto print() -> void
  {
    data.print();
  }
};