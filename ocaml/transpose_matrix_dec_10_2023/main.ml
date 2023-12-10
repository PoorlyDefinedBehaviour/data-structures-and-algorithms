(*
Given a 2D integer array matrix, return the transpose of matrix.

The transpose of a matrix is the matrix flipped over its main diagonal, 
switching the matrix's row and column indices.

Example 1:

Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
Output: [[1,4,7],[2,5,8],[3,6,9]]
Example 2:

Input: matrix = [[1,2,3],[4,5,6]]
Output: [
  [1,4],
  [2,5],
  [3,6]]
 

Constraints:

m == matrix.length
n == matrix[i].length
1 <= m, n <= 1000
1 <= m * n <= 105
-109 <= matrix[i][j] <= 109
*)
type matrix = int array array 
[@@deriving show]

(**
time O(n * m)    
space O(n * m)
*)
let transpose (matrix : matrix) : matrix =
  let rows = Array.length matrix in

  if rows = 0 then [||]
  else
    let columns= Array.length matrix.(0) in 
    
    let transposed = Array.init columns (fun _ -> Array.make rows 0) in

    matrix
    |> Array.iteri (fun i row ->
        row |> Array.iteri (fun j _ -> 
        transposed.(j).(i) <- matrix.(i).(j);));

    transposed

let%test_unit "transpose: examples" = 
  [
    ([||], [||]);
    ([|[|1;2;3|];[|4;5;6|];[|7;8;9|]|],[|[|1;4;7|];[|2;5;8|];[|3;6;9|]|]);
    ([|[|1;2;3|];[|4;5;6|]|], [|[|1;4|];[|2;5|];[|3;6|]|])
  ] |> 
  List.iter (fun (matrix, expected) -> 
    assert (expected = transpose matrix)
    )
