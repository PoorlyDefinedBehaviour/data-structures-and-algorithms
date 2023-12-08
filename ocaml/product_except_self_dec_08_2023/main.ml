(*
Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].

The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

You must write an algorithm that runs in O(n) time and without using the division operation.

Example 1:

Input: nums = [1,2,3,4]
Output: [24,12,8,6]
Example 2:

Input: nums = [-1,1,0,-3,3]
Output: [0,0,9,0,0] 

Constraints:

2 <= nums.length <= 105
-30 <= nums[i] <= 30
The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

Follow up: Can you solve the problem in O(1) extra space complexity? (The output array does not count as extra space for space complexity analysis.)
*)

(*
time O(n)
space O(1)
*)
let product_except_self (nums : int array) : int array =
  let nums_length = Array.length nums in

  let suffix, _, _ =
    Array.fold_right
      (fun num (suffix, i, product) ->
        suffix.(i) <- product;
        (suffix, i - 1, product * num))
      nums
      (Array.make nums_length 0, nums_length - 1, 1)
  in

  let nums, _, _ =
    Array.fold_left
      (fun (nums, i, product) num ->
        nums.(i) <- product * suffix.(i);
        (nums, i + 1, product * num))
      (nums, 0, 1) nums
  in
  nums

(*
time O(n)   
space O(1)
*)
let product_except_self_2 (nums : int array) : int array =
  let nums_length = Array.length nums in
  let suffix = Array.make nums_length 0 in

  let product = ref 1 in
  for i = nums_length - 1 downto 0 do
    suffix.(i) <- !product;
    product := !product * nums.(i)
  done;

  let product = ref 1 in
  for i = 0 to nums_length - 1 do
    let num = nums.(i) in
    nums.(i) <- !product * suffix.(i);
    product := !product * num
  done;

  nums

let () =
  Printf.printf "v1: example 1: %b\n"
    (product_except_self [| 1; 2; 3; 4 |] = [| 24; 12; 8; 6 |]);

  Printf.printf "v1: example 2: %b\n"
    (product_except_self [| -1; 1; 0; -3; 3 |] = [| 0; 0; 9; 0; 0 |]);

  Printf.printf "v2: example 1: %b\n"
    (product_except_self_2 [| 1; 2; 3; 4 |] = [| 24; 12; 8; 6 |]);

  Printf.printf "v2: example 2: %b\n"
    (product_except_self_2 [| -1; 1; 0; -3; 3 |] = [| 0; 0; 9; 0; 0 |])
