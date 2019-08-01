#include "utils/print.hpp"

#include "arrays/DynamicArray.hpp"

auto main() -> int
{
  DynamicArray<int> array;
  array.push(3);
  array.push(2);
  array.push(1);

  array.map([](auto const &element) -> int { return element * 2; }).print();
}
