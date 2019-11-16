#pragma once

#include <string>
#include <cstddef>

auto jenkins_one_at_a_time_hash_impl(std::string const &key) -> std::size_t
{
  std::size_t i = 0;
  uint32_t hash = 0;
  while (i != key.length())
  {
    hash += key[i++];
    hash += hash << 10;
    hash ^= hash >> 6;
  }
  hash += hash << 3;
  hash ^= hash >> 11;
  hash += hash << 15;
  return hash;
}

auto jenkins_one_at_a_time_hash_impl(char const key) -> std::size_t
{
  std::size_t i = 0;
  uint32_t hash = 0;

  hash += key;
  hash += hash << 10;
  hash ^= hash >> 6;

  hash += hash << 3;
  hash ^= hash >> 11;
  hash += hash << 15;
  return hash;
}

auto jenkins_one_at_a_time_hash = [](auto const &key) -> std::size_t {
  jenkins_one_at_a_time_hash_impl(key);
};