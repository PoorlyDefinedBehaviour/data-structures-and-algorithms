defmodule Kata do
  # time O(n)
  # space O(1) if consiredering only letters
  def is_anagram(string_a, string_b) do
    String.length(string_a) === String.length(string_b) and
      all_letters_are_used(string_a, string_b)
  end

  defp all_letters_are_used(string_a, string_b) do
    count_letter_ocurrences(string_a, string_b)
    |> Enum.all?(fn {_letter, count} -> count === 0 end)
  end

  defp count_letter_ocurrences(string_a, string_b) do
    Enum.zip(String.graphemes(string_a), String.graphemes(string_b))
    |> Enum.reduce(%{}, fn {letter_from_string_a, letter_from_string_b}, ocurrences ->
      ocurrences =
        case Map.get(ocurrences, letter_from_string_a) do
          nil -> Map.put(ocurrences, letter_from_string_a, 1)
          count -> Map.put(ocurrences, letter_from_string_a, count + 1)
        end

      case Map.get(ocurrences, letter_from_string_b) do
        nil -> Map.put(ocurrences, letter_from_string_b, -1)
        count -> Map.put(ocurrences, letter_from_string_b, count - 1)
      end
    end)
  end
end
