type 'a t = { mutable size : int; mutable buckets : 'a list array }

let create initial_capacity =
  { size = 0; buckets = Array.make initial_capacity [] }

let index_for_value set value = Hashtbl.hash value mod Array.length set.buckets

let rec insert (set : 'a t) (value : 'a) : unit =
  if set.size >= Array.length set.buckets / 90 * 100 then resize set;
  let index = index_for_value set value in
  // NOTE: duplicated values are not handled.
  set.buckets.(index) <- value :: set.buckets.(index);
  set.size <- set.size + 1

and resize (set : 'a t) =
  let entries = set.buckets in
  let new_size = if set.size = 0 then 1 else set.size * 2 in
  set.size <- 0;
  set.buckets <- Array.make new_size [];
  Array.iter
    (fun values -> List.iter (fun value -> ignore (insert set value)) values)
    entries

let mem (set : 'a t) (value : 'a) =
  let values = set.buckets.(index_for_value set value) in
  List.exists (fun v -> v = value) values

let remove (set : 'a t) (value : 'a) =
  let index = index_for_value set value in
  let values = set.buckets.(index) in
  let new_values = List.filter (fun v -> v != value) values in
  set.buckets.(index) <- new_values;
  set.size <- set.size - 1
