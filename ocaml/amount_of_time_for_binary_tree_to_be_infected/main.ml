
type tree_node = {
  value : int;
  left : tree_node option;
  right : tree_node option;
}

module IntSet = Set.Make (Int)

let traverse (adjacency_list : (int, int list) Hashtbl.t) (start : int) : int =
  let rec go infected node_value =
    (* Add the node the set of nodes infected. *)
    let infected = IntSet.add node_value infected in

    (* Find the neighbors of the current node. *)
    let neighbors = Hashtbl.find adjacency_list node_value in

    (* If the node can has at least one neighbor that can be infected. *)
    let not_infected_neighbors =
      List.filter (fun neighbor -> not (IntSet.mem neighbor infected)) neighbors
    in

    let at_least_one_neighbor_not_infected =
      List.length not_infected_neighbors > 0
    in

    (* Takes 1 unit of time to infected one or more neighbors. *)
    let time_it_took = if at_least_one_neighbor_not_infected then 1 else 0 in

    (* Calculate how long it takes for the neighbors to infected their neighbors *)
    not_infected_neighbors
    |> List.map (fun neighbor -> go infected neighbor)
    |> List.fold_left ( + ) time_it_took
  in

  go IntSet.empty start

(*
   time O(n) where n is the number of nodes.
   space O(n) where n is the number of nodes.
*)
let amount_of_time (node : tree_node option) (start : int) : int =
  (* Build a map from node value to the value of its neighbors. *)
  let adjacency_list = Hashtbl.create 0 in

  let rec go previous_node node =
    match node with
    | None -> ()
    | Some node ->
        let neighbors = ref [] in
        Option.iter
          (fun node -> neighbors := node.value :: !neighbors)
          previous_node;
        Option.iter
          (fun node -> neighbors := node.value :: !neighbors)
          node.left;
        Option.iter
          (fun node -> neighbors := node.value :: !neighbors)
          node.right;
        Hashtbl.replace adjacency_list node.value !neighbors;
        go (Some node) node.left;
        go (Some node) node.right
  in
  go None node;

  traverse adjacency_list start

let%test_unit "example 1" =
  let tree =
    {
      value = 1;
      left =
        Some
          {
            value = 5;
            left = None;
            right =
              Some
                {
                  value = 4;
                  left = Some { value = 9; left = None; right = None };
                  right = Some { value = 2; left = None; right = None };
                };
          };
      right =
        Some
          {
            value = 3;
            left = Some { value = 10; left = None; right = None };
            right = Some { value = 6; left = None; right = None };
          };
    }
  in
  assert (4 = amount_of_time (Some tree) 3)

let%test_unit "example 2" =
  let tree = { value = 1; left = None; right = None } in
  assert (0 = amount_of_time (Some tree) 1)
