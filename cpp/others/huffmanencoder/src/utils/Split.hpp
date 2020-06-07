#pragma once

#include <string>
#include <vector>
#include <cstddef>

auto split(std::string const &string, std::string delimiter) -> std::vector<std::string>
{
  std::vector<std::string> result;

  std::size_t last = 0;
  std::size_t next = 0;
  while ((next = string.find(delimiter, last)) != std::string::npos)
  {
    result.emplace_back(string.substr(last, next - last));
    last = next + 1;
  }

  result.emplace_back(string.substr(last));
  return result;
}
