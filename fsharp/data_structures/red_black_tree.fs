type Tree<'a> =
  | Black of Tree<'a> * 'a * Tree<'a>
  | Red of Tree<'a> * 'a * Tree<'a>
  | Leaf

let balance node =
  match node with
  | Black (Red (Red (left3, value3, right3), value2, right2), value1, right1) ->
    Red(Black(left3, value3, right3), value2, Black(right2, value1, right1))
  | Black (Red (left1, value1, Red (left2, value2, right2)), value3, right3) ->
    Red(Black(left1, value1, left2), value2, Black(right2, value3, right3))
  | Black (left1, value1, Red (Red (left3, value3, right3), value2, right2)) ->
    Red(Black(left1, value1, left3), value3, Black(right2, value2, right3))
  | Black (left1, value1, Red (left2, value2, Red (left3, value3, right3))) ->
    Red(Black(left1, value1, left2), value2, Black(left3, value3, right3))
  | node -> node

let insert value tree =
  let rec insertImpl value tree =
    match tree with
    | Leaf -> Red(Leaf, value, Leaf)
    | Black (left, value', right) as node ->
      if value < value' then
        balance (Black(insertImpl value left, value', right))
      else if value > value' then
        balance (Black(left, value', insertImpl value right))
      else
        node
    | Red (left, value', right) as node ->
      if value < value' then
        balance (Red(insertImpl value left, value', right))
      else if value > value' then
        balance (Red(left, value', insertImpl value right))
      else
        node

  match insertImpl value tree with
  | Leaf -> failwith "unreachable"
  | Black (left, value, right)
  | Red (left, value, right) -> Black(left, value, right)

let rec height tree =
  match tree with
  | Leaf -> 0
  | Black (left, _, right)
  | Red (left, _, right) -> 1 + max (height left) (height right)

[<EntryPoint>]
let main _ =
  let tree =
    Leaf
    |> insert 1
    |> insert 2
    |> insert 3
    |> insert 4
    |> insert 5

  printfn "%A" tree

  printfn "height %d" (height tree)

  0
