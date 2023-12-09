(*
Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:

Each row must contain the digits 1-9 without repetition.
Each column must contain the digits 1-9 without repetition.
Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
Note:

A Sudoku board (partially filled) could be valid but is not necessarily solvable.
Only the filled cells need to be validated according to the mentioned rules.

Example 1:

Input: board = 
[["5","3",".",".","7",".",".",".","."]
,["6",".",".","1","9","5",".",".","."]
,[".","9","8",".",".",".",".","6","."]
,["8",".",".",".","6",".",".",".","3"]
,["4",".",".","8",".","3",".",".","1"]
,["7",".",".",".","2",".",".",".","6"]
,[".","6",".",".",".",".","2","8","."]
,[".",".",".","4","1","9",".",".","5"]
,[".",".",".",".","8",".",".","7","9"]]
Output: true
Example 2:

Input: board = 
[["8","3",".",".","7",".",".",".","."]
,["6",".",".","1","9","5",".",".","."]
,[".","9","8",".",".",".",".","6","."]
,["8",".",".",".","6",".",".",".","3"]
,["4",".",".","8",".","3",".",".","1"]
,["7",".",".",".","2",".",".",".","6"]
,[".","6",".",".",".",".","2","8","."]
,[".",".",".","4","1","9",".",".","5"]
,[".",".",".",".","8",".",".","7","9"]]
Output: false
Explanation: Same as Example 1, except with the 5 in the top left corner being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.
 
Constraints:

board.length == 9
board[i].length == 9
board[i][j] is a digit 1-9 or '.'.
*)

let char_to_int (c : char) : int =
  match c with
  | '1' -> 1
  | '2' -> 2
  | '3' -> 3
  | '4' -> 4
  | '5' -> 5
  | '6' -> 6
  | '7' -> 7
  | '8' -> 8
  | '9' -> 9
  | _ -> assert false

(*
time O(n)
space O(1)
*)
let is_valid_sudoku (board : char array array) : bool =
  assert (Array.length board = Array.length board.(0));

  let board_size = Array.length board in

  let valid = ref true in

  let row_or_column = ref 0 in

  (* Check for duplicated numbers in 3x3 grid *)
  let grid_size = 3 in

  let grid_row_starts_at = ref 0 in

  let grid_column_starts_at = ref 0 in

  (* time O(n) *)
  while !valid && !grid_row_starts_at <= 6 do
    while !valid && !grid_column_starts_at <= 6 do
      let count = Array.make 9 0 in

      for i = 0 to 2 do
        for j = 0 to 2 do
          let value =
            board.(!grid_row_starts_at + i).(!grid_column_starts_at + j)
          in
          if value != '.' then (
            let index = char_to_int value - 1 in
            count.(index) <- count.(index) + 1;

            if count.(index) > 1 then valid := false)
        done
      done;

      grid_column_starts_at := !grid_column_starts_at + grid_size
    done;

    grid_row_starts_at := !grid_row_starts_at + grid_size;
    grid_column_starts_at := 0
  done;

  (* time O(n) *)
  while !valid && !row_or_column < board_size do
    (* Check for duplicated numbers in the current row *)
    let count = Array.make 9 0 in

    for i = 0 to board_size - 1 do
      (* A value from 1 to 9. *)
      let value = board.(!row_or_column).(i) in
      if value != '.' then (
        let index = char_to_int value - 1 in
        count.(index) <- count.(index) + 1;
        if count.(index) > 1 then valid := false)
    done;

    (* Check for duplicated numbers in the current column *)
    let count = Array.make 9 0 in

    (* time O(n) *)
    for i = 0 to board_size - 1 do
      (* A value from 1 to 9. *)
      let value = board.(i).(!row_or_column) in
      if value != '.' then (
        let index = char_to_int value - 1 in
        count.(index) <- count.(index) + 1;
        if count.(index) > 1 then valid := false)
    done;

    row_or_column := !row_or_column + 1
  done;

  !valid

let () =
  Printf.printf "example 1: should be true %b\n"
    (is_valid_sudoku
       [|
         [| '5'; '3'; '.'; '.'; '7'; '.'; '.'; '.'; '.' |];
         [| '6'; '.'; '.'; '1'; '9'; '5'; '.'; '.'; '.' |];
         [| '.'; '9'; '8'; '.'; '.'; '.'; '.'; '6'; '.' |];
         [| '8'; '.'; '.'; '.'; '6'; '.'; '.'; '.'; '3' |];
         [| '4'; '.'; '.'; '8'; '.'; '3'; '.'; '.'; '1' |];
         [| '7'; '.'; '.'; '.'; '2'; '.'; '.'; '.'; '6' |];
         [| '.'; '6'; '.'; '.'; '.'; '.'; '2'; '8'; '.' |];
         [| '.'; '.'; '.'; '4'; '1'; '9'; '.'; '.'; '5' |];
         [| '.'; '.'; '.'; '.'; '8'; '.'; '.'; '7'; '9' |];
       |]);

  Printf.printf "example 2: should be false %b\n"
    (is_valid_sudoku
       [|
         [| '8'; '3'; '.'; '.'; '7'; '.'; '.'; '.'; '.' |];
         [| '6'; '.'; '.'; '1'; '9'; '5'; '.'; '.'; '.' |];
         [| '.'; '9'; '8'; '.'; '.'; '.'; '.'; '6'; '.' |];
         [| '8'; '.'; '.'; '.'; '6'; '.'; '.'; '.'; '3' |];
         [| '4'; '.'; '.'; '8'; '.'; '3'; '.'; '.'; '1' |];
         [| '7'; '.'; '.'; '.'; '2'; '.'; '.'; '.'; '6' |];
         [| '.'; '6'; '.'; '.'; '.'; '.'; '2'; '8'; '.' |];
         [| '.'; '.'; '.'; '4'; '1'; '9'; '.'; '.'; '5' |];
         [| '.'; '.'; '.'; '.'; '8'; '.'; '.'; '7'; '9' |];
       |]);

  Printf.printf "example 3: should be false %b\n"
    (is_valid_sudoku
       [|
         [| '5'; '3'; '.'; '.'; '7'; '.'; '.'; '.'; '.' |];
         [| '6'; '.'; '.'; '1'; '9'; '5'; '.'; '.'; '.' |];
         [| '.'; '9'; '8'; '.'; '.'; '.'; '.'; '6'; '.' |];
         [| '8'; '.'; '.'; '.'; '6'; '2'; '.'; '.'; '3' |];
         [| '4'; '.'; '.'; '.'; '.'; '3'; '.'; '.'; '1' |];
         [| '7'; '8'; '.'; '.'; '2'; '.'; '.'; '.'; '6' |];
         [| '.'; '6'; '.'; '.'; '.'; '.'; '2'; '8'; '.' |];
         [| '.'; '.'; '.'; '4'; '1'; '9'; '.'; '.'; '5' |];
         [| '.'; '.'; '.'; '.'; '8'; '.'; '.'; '7'; '9' |];
       |])
