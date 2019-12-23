using System;
using System.Collections.Generic;

class Program
{
  private static int minimumCoinChange(List<int> coins, int amount, Dictionary<int, int> cache)
  {
    if (cache.ContainsKey(amount)) return cache[amount];
    if (amount == 0) return 0;

    int min = int.MaxValue;

    foreach (int coin in coins)
    {
      if (coin <= amount)
      {
        cache[amount] = minimumCoinChange(coins, amount - coin, cache);
        min = Math.Min(min, cache[amount] + 1);
      }
    }

    cache[amount] = min;
    return min;
  }

  public static void Main(string[] args)
  {
    Console.WriteLine("Minimum coins necessary => {0}",
      minimumCoinChange(new List<int> { 1, 2, 5 }, 11, new Dictionary<int, int>()));
  }
}