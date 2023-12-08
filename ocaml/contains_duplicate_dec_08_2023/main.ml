(*
   Given an integer array nums,
         return true if any value appears at least twice in the array,
         and return false if every element is distinct.

      Example 1:

      Input: nums = [1,2,3,1]
      Output: true
      Example 2:

      Input: nums = [1,2,3,4]
      Output: false
      Example 3:

      Input: nums = [1,1,1,3,3,4,3,2,4,2]
      Output: true
*)

(* time O(n) *)
(* space O(n) *)
let contains_duplicate nums =
  let rec go seen nums =
    match nums with
    | [] -> false
    | head :: tail ->
        if Hashtbl.mem seen head then true
        else (
          Hashtbl.replace seen head ();
          (go [@tailcall]) seen tail)
  in
  go (Hashtbl.create 0) nums

let () =
  Printf.printf "example 1: %b\n" (contains_duplicate [ 1; 2; 3; 1 ]);
  Printf.printf "example 2: %b\n" (contains_duplicate [ 1; 2; 3; 4 ]);
  Printf.printf "example 3: %b\n"
    (contains_duplicate [ 1; 1; 1; 3; 3; 4; 3; 2; 4; 2 ])
