using System;
using System.Collections.Generic;

class Program
{
  private static int minimumCoinChange(List<int> coins, int amount)
  {
    if (amount == 0) return 0;

    int min = int.MaxValue;

    foreach (int coin in coins)
      if (coin <= amount)
        min = Math.Min(min, minimumCoinChange(coins, amount - coin) + 1);

    return min;
  }

  public static void Main(string[] args)
  {
    Console.WriteLine("Minimum coins necessary => {0}", minimumCoinChange(new List<int> { 1, 2, 5 }, 11));
  }
}