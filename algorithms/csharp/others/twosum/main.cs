using System.Collections.Generic;
using System;

class Program
{
  /**
  * time O(n)
  * space O(n)
  */
  private static Tuple<int, int> two_sum(List<int> numbers, int target)
  {
    Dictionary<int, int> cache = new Dictionary<int, int>();

    for (int i = 0; i < numbers.Count; ++i)
      cache.Add(numbers[i], i);

    for (int i = 0; i < cache.Count; ++i)
    {
      int number = Math.Abs(target - numbers[i]);

      if (cache.ContainsKey(number)
          && number + numbers[i] == target
          && i != cache[number])
        return Tuple.Create(i, cache[number]);
    }

    return Tuple.Create(-1, -1);
  }

  public static void Main(string[] args)
  {
    /*Given an array of integers, return indices of the two numbers such that they add up to a specific target.
    You may assume that each input would have exactly one solution, and you may not use the same element twice.

    Example:
    Given nums = [2, 7, 11, 15], target = 9,
    Because nums[0] + nums[1] = 2 + 7 = 9,
    return [0, 1].
    */
    Console.WriteLine("Should return (0, 1) => {0}", two_sum(new List<int> { 2, 7, 11, 15 }, 9));
    Console.WriteLine("Should return (-1, -1) => {0}", two_sum(new List<int> { 2, 14, 11, 15 }, 9));
  }
}

