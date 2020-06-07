
using System.Linq;
using System.Collections.Generic;
using System;

class Program
{
  static Func<T, U> memoize<T, U>(Func<T, U> function)
  {
    Dictionary<T, U> cache = new Dictionary<T, U>();

    return (T value) =>
     {
       if (cache.ContainsKey(value))
       {
         Console.WriteLine("Value {0} is in cache", value);
         return cache[value];
       }

       Console.WriteLine("Calculating value {0}", value);
       U result = function(value);
       cache.Add(value, result);
       return result;
     };
  }

  public static void Main(string[] args)
  {
    Func<int, int> fibonacci = (n) => n;
    fibonacci = memoize<int, int>((n) => n <= 1 ? 1 : fibonacci(n - 1) + fibonacci(n - 2));

    Enumerable.Range(1, 10)
              .ToList()
              .ForEach((number) =>
                Console.WriteLine("Fibnacci {0} => {1}", number, fibonacci(number))
              );
  }
}

