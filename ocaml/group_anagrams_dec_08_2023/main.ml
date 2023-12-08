(*
Given an array of strings strs, group the anagrams together. You can return the answer in any order.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, 
typically using all the original letters exactly once. 

Example 1:

Input: strs = ["eat","tea","tan","ate","nat","bat"]
Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
Example 2:

Input: strs = [""]
Output: [[""]]
Example 3:

Input: strs = ["a"]
Output: [["a"]]
 

Constraints:

1 <= strs.length <= 104
0 <= strs[i].length <= 100
strs[i] consists of lowercase English letters.
*)

(* time O(n) *)
(* space O(1) *)
let count_letters str =
  (* strs[i] consists of lowercase English letters. *)
  let count = Array.make 26 0 in

  let a_ascii_code = Char.code 'a' in

  String.iter
    (fun char ->
      let index = Char.code char - a_ascii_code in
      count.(index) <- count.(index) + 1)
    str;

  count

(*
   time O(n * m)
   space O(n * m)
*)
let group_anagrams strs =
  let groups = Hashtbl.create 0 in

  (* time O(n) *)
  List.iter
    (fun str ->
      let count = count_letters str in
      match Hashtbl.find_opt groups count with
      | None -> Hashtbl.replace groups count [ str ]
      | Some list -> Hashtbl.replace groups count (str :: list))
    strs;
  groups

let print_hashtbl table =
  print_endline "--- hashtable ---";
  Hashtbl.iter
    (fun _ list ->
      print_endline "--- row ---";
      List.iter print_endline list)
    table

let () =
  print_hashtbl (group_anagrams [ "eat"; "tea"; "tan"; "ate"; "nat"; "bat" ]);
  print_hashtbl (group_anagrams [ "" ]);
  print_hashtbl (group_anagrams [ "a" ])
