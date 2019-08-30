#pragma once

namespace utils
{
static auto binary_search = [](auto const &array, size_t start, size_t end, auto const &element) -> size_t {
  if (end >= start)
  {
    size_t mid = start + (end - start) / 2;

    if (array[mid] == element)
      return mid;

    if (array[mid] > element)
      return binary_search(array, start, mid - 1, element);

    return binary_search(array, mid + 1, end, element);
  }

  return -1;
};
} // namespace utils