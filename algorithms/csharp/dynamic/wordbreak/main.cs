using System.Collections.Generic;
using System;

class Program
{
  private static bool wordBreak(string text, Dictionary<string, bool> dictionary)
  {
    bool[] table = new bool[text.Length + 1];
    Array.Fill(table, false);
    table[0] = true;

    for (int i = 0; i < table.Length; ++i)
    {
      if (!table[i]) continue;

      for (int j = i + 1; j < table.Length; ++j)
      {
        var word = text.Substring(i, j - i);
        if (dictionary.ContainsKey(word))
          table[j] = true;
      }
    }

    return table[table.Length - 1];
  }

  public static void Main(string[] args)
  {
    Dictionary<string, bool> dictionary = new Dictionary<string, bool>();
    dictionary.Add("apple", true);
    dictionary.Add("pen", true);

    Console.WriteLine(wordBreak("applepenapple", dictionary));
  }
}