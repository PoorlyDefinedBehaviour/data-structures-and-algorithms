module AssociationList

type T<'key, 'value> = ('key * 'value) list

// time O(1)
let empty = []

// time O(1)
let insert key value assocList = (key, value) :: assocList

// time O(n)
let find key assocList =
    List.tryFind (fun (key', _) -> key' = key) assocList
    |> Option.map snd

// time O(n)
let contains key assocList = Option.isSome (find key assocList)

// time O(n)
let remove key assocList =
    List.filter (fun (key', _) -> key' <> key) assocList
