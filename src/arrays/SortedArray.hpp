#include <stdexcept>
#include <memory>
#include <functional>

#include "DynamicArray.hpp"
#include "../utils/BinarySearch.hpp"

template <typename T>
class SortedArray
{
private:
  DynamicArray<T> data;

public:
  auto raw() -> T * { return data.raw(); }

  auto length() -> size_t { return data.length(); }

  auto operator[](size_t index) -> T &
  {
    if (index > size - 1)
      throw std::out_of_range("Invalid index");

    return data[index];
  }

  auto first() -> T & { return data[0]; }
  auto last() -> T & { return data[size - 1]; }
  auto first_copy() -> T { return data[0]; }
  auto last_copy() -> T { return data[size - 1]; }

  auto shift() -> T
  {
    if (size == 0)
      throw std::logic_error("Array is empty");
  }

  auto pop() -> T
  {
    if (data.is_empty())
      throw std::logic_error("Array is empty");

    return data.pop();
  }

  auto push(T const &element) -> void
  {
    if (size == 0)
    {
      data[size++] = element;
      return;
    }

    data.push(element);
    data.sort(std::greater<T>());
  };

  auto remove_at(size_t index) -> void
  {
    data.remove_at(index);
  }

  auto includes(T const &element) -> bool
  {
    return data.includes(element);
  }

  auto index_of(T const &element) -> size_t
  {
    return utils::binary_search(data, 0, size - 1, element);
  }

  template <typename lambda>
  auto find(lambda const &fn) -> size_t
  {
    return data.find(fn);
  }

  template <typename lambda>
  auto map(lambda const &fn) -> Array<T>
  {
    return data.map(fn);
  }

  template <typename lambda>
  auto filter(lambda const &fn) -> Array<T>
  {
    return data.filter(fn);
  }

  template <typename lambda>
  auto reduce(lambda const &fn) -> T
  {
    return data.reduce(fn);
  }

  auto print() -> void
  {
    data.print();
  }
};