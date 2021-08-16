[<EntryPoint>]
let main _ =
  let xs =
    AssociationList.empty
    |> AssociationList.insert "a" 1
    |> AssociationList.insert "b" 2

  printfn "%A" xs // [("b", 2); ("a", 1)]

  printfn "%A" (AssociationList.find "a" xs) // Some 1

  printfn "%A" (AssociationList.find "c" xs) // None

  printfn "%A" (AssociationList.contains "c" xs) // false

  printfn "%A" (AssociationList.contains "b" xs) // true

  let xs = AssociationList.remove "a" xs

  printfn "%A" xs // [("b", 2)]

  printfn "%A" (AssociationList.find "a" xs) // None

  printfn "%A" (AssociationList.find "c" xs) // None

  printfn "%A" (AssociationList.contains "c" xs) // false

  printfn "%A" (AssociationList.contains "b" xs) // true

  0
