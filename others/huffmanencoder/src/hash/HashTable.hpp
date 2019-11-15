#pragma once

#include <string>
#include <vector>
#include <cstddef>
#include <stdexcept>
#include <numeric>
#include <functional>
#include <utility>
#include <algorithm>

#include "functions/NaiveHash.hpp"
#include "../utils/Split.hpp"

template <typename KeyType, typename ValueType>
class HashTable
{
private:
  struct Entry
  {
    KeyType key;
    ValueType value;

    Entry(KeyType const &key, ValueType const &value)
        : key(key), value(value) {}
  };

  using HashFunction = std::function<std::size_t(KeyType const &)>;
  HashFunction hash = naive_hash;

  std::size_t capacity_;
  std::size_t size_ = 0;

  std::vector<Entry *> elements;

  auto linear_prob(KeyType const &key) -> std::size_t
  {
    std::size_t index = hash(key) % capacity_;

    while (elements[index] != nullptr && elements[index]->key != key)
      index = (index + 1) % capacity_;

    return index;
  }

public:
  HashTable(std::size_t capacity_)
  {
    this->capacity_ = capacity_ * 4 / 3;

    elements.reserve(this->capacity_);
    for (std::size_t i = 0; i < this->capacity_; ++i)
      elements.emplace_back(nullptr);
  }

  ~HashTable()
  {
    for (std::size_t i = 0; i < capacity_; ++i)
      if (elements[i] != nullptr)
        delete elements[i];
  }

  auto size() const -> std::size_t { return size_; }
  auto capacity() const -> std::size_t { return capacity_; }

  auto set_hash_function(HashFunction const &hash) -> HashTable<KeyType, ValueType> &
  {
    this->hash = hash;
    return *this;
  }

  auto set(KeyType const &key, ValueType const &value) -> HashTable<KeyType, ValueType> &
  {
    if (size_ == capacity_)
      throw std::runtime_error("HashTable is full");

    std::size_t const index = linear_prob(key);

    if (elements[index])
    {
      elements[index]->key = key;
      elements[index]->value = value;
    }
    else
    {
      elements[index] = new Entry(key, value);
      size_ += 1;
    }

    return *this;
  }

  auto get(KeyType const &key) -> ValueType const &
  {
    std::size_t const index = linear_prob(key);

    if (!elements[index])
      throw std::runtime_error("Unknown key at HashTable.get() => " + key);

    return elements[index]->value;
  }

  auto remove(KeyType const &key) -> HashTable<KeyType, ValueType> &
  {
    std::size_t const index = linear_prob(key);
    if (!elements[index])
      throw std::runtime_error("Unknown key at HashTable.remove() => " + key);

    delete elements[index];
    elements[index] = nullptr;
    size_ -= 1;

    return *this;
  }

  auto has(KeyType const &key) -> bool
  {
    return elements[linear_prob(key)] != nullptr;
  }

  auto get_collisions() const -> std::vector<std::pair<std::size_t, std::string>>
  {
    std::vector<std::pair<std::size_t, std::string>> collisions;
    collisions.reserve(size_);

    for (std::size_t i = 0; i < capacity_; ++i)
    {
      Entry *entry = elements[i];

      if (entry == nullptr)
        continue;

      std::size_t const index = hash(entry->key) % capacity_;

      auto iterator = std::find_if(std::begin(collisions), std::end(collisions), [&](auto const &e) {
        return e.first == index;
      });

      if (iterator != std::end(collisions))
        iterator->second = iterator->second + "," + entry->key;
      else
        collisions.emplace_back(std::make_pair(index, entry->key));
    }

    return collisions;
  }

  auto keys() const -> std::vector<std::string>
  {
    std::vector<KeyType> result;
    result.reserve(size_);

    for (auto const &entry : elements)
      if (entry != nullptr)
        result.emplace_back(entry->key);

    return result;
  }

  auto values() const -> std::vector<ValueType>
  {
    std::vector<ValueType> result;
    result.reserve(size_);

    for (auto const &entry : elements)
      if (entry != nullptr)
        result.emplace_back(entry->value);

    return result;
  }

  auto entries() const -> std::vector<std::pair<KeyType, ValueType>>
  {
    std::vector<std::pair<KeyType, ValueType>> result;
    result.reserve(size_);

    for (auto const &entry : elements)
      if (entry != nullptr)
        result.emplace_back(std::make_pair(entry->key, entry->value));

    return result;
  }
};

template <typename KeyType, typename ValueType>
auto show_table_entries(std::string const &table_name, HashTable<KeyType, ValueType> const &table) -> void
{
  Console::println("<---", table_name, "--->");

  for (auto const &[key, value] : table.entries())
    Console::println(key, ":", value);

  Console::println("</---", table_name, "--->");
}

template <typename KeyType, typename ValueType>
auto show_table_collisions(std::string const &table_name, HashTable<KeyType, ValueType> const &table) -> void
{
  Console::println("<---", table_name, "--->");

  for (auto const &[key, value] : table.get_collisions())
    Console::println(key, ":", value);

  Console::println("</---", table_name, "--->");
}

auto create_word_frequency_table(std::string const &text) -> HashTable<std::string, std::size_t>
{
  std::vector<std::string> words = split(text, " ");

  HashTable<std::string, std::size_t> table(words.size());

  for (auto const &word : words)
  {
    if (table.has(word))
      table.set(word, table.get(word) + 1);
    else
      table.set(word, 1);
  }

  return table;
}