defmodule Solution do
  import Bitwise

  # time O(3n) -> O(n)
  # space O(n)
  def run(numbers) do
    length(Enum.filter(numbers, fn x -> (String.length(to_string(x)) &&& 1) === 0 end))
  end
end

# Given an array nums of integers,
# return how many of them contain an even number of digits.
# Example:
# Input: nums = [12,345,2,6,7896]
# Output: 2
# Explanation:
# 12 contains 2 digits (even number of digits).
# 345 contains 3 digits (odd number of digits).
# 2 contains 1 digit (odd number of digits).
# 6 contains 1 digit (odd number of digits).
# 7896 contains 4 digits (even number of digits).
# Therefore only 12 and 7896 contain an even number of digits.
IO.puts(Solution.run([12, 345, 2, 6, 7896]))
