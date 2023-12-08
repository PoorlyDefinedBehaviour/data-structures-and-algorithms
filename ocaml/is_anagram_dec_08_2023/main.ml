(*
Given two strings s and t, return true if t is an anagram of s, and false otherwise.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, 
typically using all the original letters exactly once.

Example 1:

Input: s = "anagram", t = "nagaram"
Output: true
Example 2:

Input: s = "rat", t = "car"
Output: false

Constraints:

1 <= s.length, t.length <= 5 * 104
s and t consist of lowercase English letters.

*)

(* time O(n) *)
(* space O(1) *)
let is_anagram s t =
  if String.length s <> String.length t then false
  else
    (* s and t consist of lowercase English letters so we only an array of 26 positions *)
    (* space O(1) *)
    let count = Array.make 26 0 in

    let a_ascii_code = Char.code 'a' in

    (* time O(n) *)
    for i = 0 to String.length s - 1 do
      let s_index = Char.code s.[i] - a_ascii_code in
      count.(s_index) <- count.(s_index) + 1;

      let t_index = Char.code t.[i] - a_ascii_code in
      count.(t_index) <- count.(t_index) - 1
    done;

    (* time O(n) *)
    Array.for_all (fun n -> n = 0) count

let sort_string s =
  let n = String.length s in
  let array = Array.init n (fun i -> s.[i]) in
  Array.sort Char.compare array;
  String.init n (fun i -> array.(i))

(* time O(n log n) + O(n log m) *)
(* space O(n + m) *)
let is_anagram_2 s t = sort_string s = sort_string t

let () =
  Printf.printf "is_anagram example 1: %b\n" (is_anagram "anagram" "nagaram");
  Printf.printf "is_anagram example 2: %b\n" (is_anagram "rat" "car");
  Printf.printf "is_anagram2 example 1: %b\n" (is_anagram_2 "anagram" "anagram");
  Printf.printf "is_anagram2 example 2: %b\n" (is_anagram_2 "rat" "car")
