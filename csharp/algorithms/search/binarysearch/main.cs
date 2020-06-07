using System.Linq;
using System.Collections.Generic;
using System;

class Program
{
  private static int binary_search_impl(
    List<int> array,
    int start,
    int end,
    int target)
  {
    if (start > end)
      return -1;

    int middle_index = (start + end) / 2;
    int value = array[middle_index];

    if (value == target)
      return middle_index;

    return target < value
    ? binary_search_impl(array, start, middle_index - 1, target)
    : binary_search_impl(array, middle_index + 1, end, target);
  }

  private static int binary_search(List<int> array, int target)
    => binary_search_impl(array, 0, array.Count - 1, target);

  public static void Main(string[] args)
  {
    List<int> numbers = Enumerable.Range(1, 10).ToList();

    Console.WriteLine("-1 -> {0}", binary_search(numbers, -1));
    Console.WriteLine("1 -> {0}", binary_search(numbers, 1));
    Console.WriteLine("5 -> {0}", binary_search(numbers, 5));
    Console.WriteLine("100 -> {0}", binary_search(numbers, 100));
  }
}

