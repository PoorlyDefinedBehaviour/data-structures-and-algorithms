#pragma once

#include <string>
#include <cstddef>

auto murmur_oaat64_impl(std::string const &key) -> std::size_t
{
  uint64_t h(525201411107845655ull);
  for (char const character : key)
  {
    h ^= character;
    h *= 0x5bd1e9955bd1e995;
    h ^= h >> 47;
  }
  return h;
}

auto murmur_oaat64_impl(char const key) -> std::size_t
{
  uint64_t h(525201411107845655ull);

  h ^= key;
  h *= 0x5bd1e9955bd1e995;
  h ^= h >> 47;

  return h;
}

auto murmur_oaat64 = [](auto const &key) -> std::size_t {
  return murmur_oaat64_impl(key);
};