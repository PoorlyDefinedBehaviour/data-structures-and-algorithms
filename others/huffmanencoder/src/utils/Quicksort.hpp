#pragma once

auto quicksort = [](auto &array, const int &start, const int &end, auto const &predicate) -> void {
  if (start >= end)
    return;

  static auto swap = [](auto &a, auto &b) -> void {
    auto temp = std::move(a);
    a = std::move(b);
    b = std::move(temp);
  };

  int pivot_index = start;
  auto const &pivot_value = array[end];

  for (int i = start; i < end; ++i)
  {
    if (predicate(array[i], pivot_value))
    {
      swap(array[i], array[pivot_index]);
      ++pivot_index;
    }
  }

  swap(array[pivot_index], array[end]);

  quicksort(array, start, pivot_index - 1, predicate);
  quicksort(array, pivot_index + 1, end, predicate);
};