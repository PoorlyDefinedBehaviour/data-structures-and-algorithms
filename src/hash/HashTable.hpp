#pragma once

#include "../arrays/StaticArray.hpp"

#include <string>
#include <stdexcept>

/**
 * No buckets because i'm lazy
 */
template <typename T, size_t size>
class HashTable
{
private:
  StaticArray<T, size> data;

  auto hash(std::string const &key) const -> size_t
  {
    size_t buffer = 0;
    for (auto c : key)
    {
      buffer += (size_t)c;
    }

    return buffer % size;
  }

public:
  auto capacity() const -> size_t { return size; }

  auto set(std::string const &key, T const &value) -> void
  {
    data[hash(key)] = value;
  }

  auto get(std::string const &key) -> const
  {
    return data[hash(key)];
  }

  auto includes(T const &value) -> bool
  {
    return data.includes(value);
  }
};