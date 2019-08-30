#pragma once

namespace utils
{
static auto raw_array_copy = [](auto *from, auto *to, size_t start, size_t end) -> void {
  size_t index = 0;
  while (start != end)
  {
    to[index] = from[start];
    ++start;
    ++index;
  }
};
} // namespace utils
