#pragma once

#include "./Swap.hpp"

namespace utils
{
static auto quicksort = [](auto &array, size_t const &start, size_t const &end, auto const &fn) -> void {
  if (start >= end)
    return;

  size_t pivot_index = start;
  auto const &pivot_value = array[end];

  for (size_t i = start; i < end; ++i)
  {
    if (fn(array[i], pivot_value))
    {
      utils::swap(array[i], array[pivot_index]);
      ++pivot_index;
    }
  }

  utils::swap(array[pivot_index], array[end]);

  quicksort(array, start, pivot_index - 1, fn);
  quicksort(array, pivot_index + 1, end, fn);
};
}