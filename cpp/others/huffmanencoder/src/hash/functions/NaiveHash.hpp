#pragma once

#include <numeric>
#include <string>
#include <cstddef>

auto naive_hash_impl(std::string const &key) -> std::size_t
{
  return std::reduce(std::begin(key),
                     std::end(key),
                     0,
                     [](std::size_t accum, char caracter) {
                       return accum + static_cast<std::size_t>(caracter);
                     });
}

auto naive_hash_impl(char const key) -> std::size_t
{
  return static_cast<std::size_t>(key);
}

auto naive_hash = [](auto const &key) -> std::size_t {
  return naive_hash_impl(key);
};
