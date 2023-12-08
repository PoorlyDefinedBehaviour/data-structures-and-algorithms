(*
Given an integer array nums and an integer k, return the k most frequent elements. 
You may return the answer in any order.

Example 1:

Input: nums = [1,1,1,2,2,3], k = 2
Output: [1,2]
Example 2:

Input: nums = [1], k = 1
Output: [1]
 
Constraints:

1 <= nums.length <= 105
-104 <= nums[i] <= 104
k is in the range [1, the number of unique elements in the array].
It is guaranteed that the answer is unique.
 
Follow up: Your algorithm's time complexity must be better than O(n log n), where n is the array's size.   
*)

(*
   time O(n)
   space O(n)
*)
let count_nums nums =
  let count = Hashtbl.create 0 in

  List.iter
    (fun num ->
      match Hashtbl.find_opt count num with
      | None -> Hashtbl.replace count num 1
      | Some n -> Hashtbl.replace count num (n + 1))
    nums;

  count

(*
time O(n)   
space O(n)
*)
let take_n n xs =
  let rec go out out_length xs =
    match xs with
    | [] -> (out_length, out)
    | head :: tail ->
        if out_length < n then go (head :: out) (out_length + 1) tail
        else (out_length, out)
  in
  go [] 0 xs

(*
time O(n)   
space O(n)
*)
let take_n_from_matrix n xs =
  let rec go out out_length xs =
    match xs with
    | [] -> out
    | list :: rest ->
        if out_length < n then
          let length, items = take_n (n - out_length) list in
          (* NOTE: append could be a problem *)
          go (List.append out items) (out_length + length) rest
        else out
  in
  go [] 0 xs

let print_list xs =
  List.iter (Printf.printf "%d ") xs;
  print_endline ""

(*
time O(n)   
space O(n)
*)
let top_k_frequent nums (k : int) =
  (*
  time O(n)   
  space O(n)
  *)
  let count = count_nums nums in

  (* space O(n) *)
  let buckets = Array.init (List.length nums) (fun _ -> []) in

  (*time O(n)   *)
  Hashtbl.iter
    (fun num frequency ->
      let index = frequency - 1 in
      buckets.(index) <- num :: buckets.(index))
    count;

  take_n_from_matrix k (List.rev (Array.to_list buckets))

let () =
  (* [2;1] *)
  print_list (top_k_frequent [ 1; 1; 1; 2; 2; 2; 3 ] 2);

  (* [1] *)
  print_list (top_k_frequent [ 1 ] 1)
