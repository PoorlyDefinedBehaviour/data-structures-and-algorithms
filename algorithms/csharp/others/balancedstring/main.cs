using System.Diagnostics;
using System.Collections.Generic;
using System;

class Program
{
  private static bool is_opening(char character)
      => character == '(' || character == '[' || character == '{';

  private static bool is_match(char left, char right)
  {
    if (left == '(') return right == ')';
    if (left == '[') return right == ']';
    if (left == '{') return right == '}';
    return false;
  }

  private static bool is_balanced(String sequence)
  {
    Stack<char> stack = new Stack<char>();

    foreach (var character in sequence)
    {
      if (is_opening(character))
        stack.Push(character);
      else if (stack.Count == 0 || !is_match(stack.Pop(), character))
        return false;
    }

    return stack.Count == 0;
  }

  public static void Main(string[] args)
  {
    Debug.Assert(is_balanced("([{}])"));
    Debug.Assert(!is_balanced("([{])"));
    Debug.Assert(is_balanced(""));
    Debug.Assert(!is_balanced("([{}]"));
    Debug.Assert(!is_balanced("[{}])"));
    Debug.Assert(!is_balanced("123"));
    Debug.Assert(!is_balanced("invalidstring"));
  }
}

