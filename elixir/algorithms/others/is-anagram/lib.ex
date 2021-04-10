defmodule Kata do
  def is_anagram(string_a, string_b) do
    String.length(string_a) === String.length(string_b) and
      all_letters_are_used(string_a, string_b)
  end

  defp all_letters_are_used(string_a, string_b) do
    ocurrences =
      count_letter_ocurrences(string_a)
      |> decrement_letter_ocurrences(string_b)

    case ocurrences do
      {:error, :missing_letter} ->
        false

      ocurrences ->
        ocurrences
        |> Enum.all?(fn {_letter, count} -> count === 0 end)
    end
  end

  defp count_letter_ocurrences(string) do
    string
    |> String.graphemes()
    |> Enum.reduce(%{}, fn letter, ocurrences ->
      case Map.get(ocurrences, letter) do
        nil -> Map.put(ocurrences, letter, 1)
        count -> Map.put(ocurrences, letter, count + 1)
      end
    end)
  end

  defp decrement_letter_ocurrences(ocurrences, string) do
    String.graphemes(string)
    |> Enum.reduce_while(ocurrences, fn letter, ocurrences ->
      case Map.get(ocurrences, letter) do
        nil -> {:halt, {:error, :missing_letter}}
        count -> {:cont, Map.put(ocurrences, letter, count - 1)}
      end
    end)
  end
end
