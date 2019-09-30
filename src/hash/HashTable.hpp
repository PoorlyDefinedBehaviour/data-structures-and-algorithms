#pragma once

#include "../lists/LinkedList.hpp"
#include "../arrays/StaticArray.hpp"
#include <string>
#include <vector>
#include <iostream>

template <typename T>
struct Entry
{
  std::string key;
  T data;
};

template <typename T>
class HashTable
{
private:
  static size_t constexpr size = 10000;
  StaticArray<LinkedList<Entry<T>>, size> entries;

  auto hash(std::string const &key) const -> size_t
  {
    size_t buffer = 0;
    for (auto c : key)
    {
      buffer = buffer * 37 + static_cast<size_t>(c);
    }
    return buffer % size;
  }

public:
  auto capacity() const -> size_t { return size; }

  auto set(std::string const &key, T const &value) -> HashTable<T> &
  {
    auto &list = entries[hash(key)];
    list.insert({key, value});
    return *this;
  }

  auto get(std::string const &key) -> T const &
  {
    auto list = entries[hash(key)];
    auto element = list.find([&key](auto const &node) -> bool { return node.key == key; });
    if (element == nullptr)
    {
      throw std::logic_error("No element with specified key was found");
    }
    return element->data.data;
  }

  auto remove(std::string const &key) -> HashTable<T> &
  {
    auto &list = entries[hash(key)];
    size_t index = list.find_index([&key](auto const &node) -> bool { return node.key == key; });
    if (index != -1)
    {
      list.remove_at(index);
    }
    return *this;
  }

  auto has(std::string const &key) -> bool
  {
    auto list = entries[hash(key)];

    return !list.is_empty() && list.find([&key](auto const &node) -> bool { return node.key == key; }) != nullptr;
  }
};