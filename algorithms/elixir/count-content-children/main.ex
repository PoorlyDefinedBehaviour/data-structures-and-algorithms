defmodule Solution do
  def countContentChildren(_cookieMap, _childrenMap, -1, _j, count) do
    count
  end

  def countContentChildren(_cookieMap, _childrenMap, _i, -1, count) do
    count
  end

  def countContentChildren(cookieMap, childrenMap, i, j, count) do
    cond do
      Map.get(cookieMap, i) >= Map.get(childrenMap, j) ->
        countContentChildren(cookieMap, childrenMap, i - 1, j - 1, count + 1)

      true ->
        countContentChildren(cookieMap, childrenMap, i, j - 1, count)
    end
  end

  def run(children, cookies) do
    childrenMap = Stream.zip(Stream.iterate(0, &(&1 + 1)), children) |> Enum.into(%{})

    cookieMap = Stream.zip(Stream.iterate(0, &(&1 + 1)), cookies) |> Enum.into(%{})

    countContentChildren(cookieMap, childrenMap, length(cookies) - 1, length(children) - 1, 0)
  end
end

IO.puts("[1, 2, 3], [1, 1] -> #{Solution.run([1, 2, 3], [1, 1])}")

IO.puts("[1,2], [1,2,3] -> #{Solution.run([1, 2], [1, 2, 3])}")
