defmodule Kata do
  @moduledoc """
  Give a string s, count the number of non-empty (contiguous) substrings that have the same number of 0's and 1's,
  and all the 0's and all the 1's in these substrings are grouped consecutively.

  Substrings that occur multiple times are counted the number of times they occur.

  Example 1:

  Input: "00110011"
  Output: 6
  Explanation: There are 6 substrings that have equal number of consecutive 1's and 0's: "0011", "01", "1100", "10", "0011", and "01".

  Notice that some of these substrings repeat and are counted the number of times they occur.

  Also, "00110011" is not a valid substring because all the 0's (and 1's) are not grouped together.

  Example 2:

  Input: "10101"
  Output: 4
  Explanation: There are 4 substrings: "10", "01", "10", "01" that have equal number of consecutive 1's and 0's.

  Note:
  s.length will be between 1 and 50,000.
  s will only consist of "0" or "1" characters.
  """
  defp count_binary_substrings(graphemes, count, previous, current) do
    case graphemes do
      [] ->
        count

      [_] ->
        count + min(previous, current)

      [current_character, next_character | _] ->
        if current_character == next_character do
          count_binary_substrings(tl(graphemes), count, previous, current + 1)
        else
          count_binary_substrings(tl(graphemes), count + min(previous, current), current, 1)
        end
    end
  end

  @doc """
  time O(n)
  space O(1)
  """
  def count_binary_substrings(string) do
    count_binary_substrings(String.graphemes(string), 0, 0, 1)
  end
end

defmodule KataTest do
  @moduledoc false

  use ExUnit.Case
  doctest Kata

  test "count_binary_substrings/1" do
    assert Kata.count_binary_substrings("00110011") == 6
    assert Kata.count_binary_substrings("10101") == 4
  end
end
