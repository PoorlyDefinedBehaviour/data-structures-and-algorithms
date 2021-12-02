use std::collections::{HashMap, HashSet};

// 332. Reconstruct Itinerary
//
// You are given a list of airline tickets where tickets[i] = [fromi, toi] represent the departure and the arrival airports of one flight.
// Reconstruct the itinerary in order and return it.
//
// All of the tickets belong to a man who departs from "JFK", thus, the itinerary must begin with "JFK".
// If there are multiple valid itineraries, you should return the itinerary that has the smallest lexical order when read as a single string.
//
// For example, the itinerary ["JFK", "LGA"] has a smaller lexical order than ["JFK", "LGB"].
//
// You may assume all tickets form at least one valid itinerary. You must use all the tickets once and only once.
//
// Example 1:
//
// ┌───┐    ┌───┐    ┌───┐
// │MUC├────►LHR├────►SFO│
// └─▲─┘    └───┘    └─┬─┘
//   │                 │
//   │                 │
// ┌─┴─┐             ┌─▼─┐
// │JFK│             │SJC│
// └───┘             └───┘
//
// Input: tickets = [["MUC","LHR"],["JFK","MUC"],["SFO","SJC"],["LHR","SFO"]]
// Output: ["JFK","MUC","LHR","SFO","SJC"]
//
// Constraints:
//
//     1 <= tickets.length <= 300
//     tickets[i].length == 2
//     fromi.length == 3
//     toi.length == 3
//     fromi and toi consist of uppercase English letters.
//     fromi != toi

// time O(v + e)
//  where v is the number of vertices in the adjacency list
//  and e is the number of edges.
// space O(n) where n is the number of tickets.
pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut tickets = tickets;

    tickets.sort();

    let mut adjacency_list = HashMap::new();

    for ticket in &mut tickets {
        let to = ticket.pop().unwrap();
        let from = ticket.pop().unwrap();

        let neighbors = adjacency_list.entry(from.clone()).or_insert(vec![]);

        neighbors.push(to);
    }

    fn dfs(
        current: &String,
        visited: &mut HashSet<String>,
        adjacency_list: &HashMap<String, Vec<String>>,
        path: &mut Vec<String>,
        path_len: usize,
    ) -> bool {
        if path.len() == path_len {
            return true;
        }

        for neighbors in adjacency_list.get(current) {
            for neighbor in neighbors {
                let edge = format!("{}_{}", current, neighbor);

                if visited.contains(&edge) {
                    continue;
                }

                visited.insert(edge.clone());

                path.push(neighbor.clone());

                if dfs(neighbor, visited, adjacency_list, path, path_len) {
                    return true;
                }

                // If we didn't find a solution after inserting [neighbor] in the [path],
                // remove [neighbor] from the [path] because it is not in the right position.
                // This is known as backtracking.
                path.pop();
            }
        }

        false
    }

    // The tickets owner always starts at JFK,
    // we add 1 to take that into account since
    // the man will travel from JFK to another location
    // at least once.
    let path_len = tickets.len() + 1;

    for from in adjacency_list.keys() {
        // A set of visited edges that we will use to check if an edge
        // has already been visited. If an edge has already been visited,
        // we won't visit it again.
        //
        // Given the adjacency list:
        //  {
        //    "MUC": ["LHR"],
        //    "JFK": ["MUC"],
        //    "LHR": ["SFO"],
        //    "SFO": ["SJC"],
        //  }
        //
        // [visited] will contain:
        // {JFK_MUC} after we visited MUC from JFK
        // {JFK_MUC, MUC_LHR} after we visited MUC from JFK and LHR FROM MUC.
        let mut visited = HashSet::new();

        let mut path = Vec::with_capacity(path_len);

        // // All of the tickets belong to a man who departs from "JFK", thus, the itinerary must begin with "JFK".
        path.push(String::from("JFK"));

        if dfs(from, &mut visited, &adjacency_list, &mut path, path_len) {
            return path;
        }
    }

    unreachable!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ind_itinerary() {
        let tests = vec![
            (
                vec![
                    vec![String::from("MUC"), String::from("LHR")],
                    vec![String::from("JFK"), String::from("MUC")],
                    vec![String::from("SFO"), String::from("SJC")],
                    vec![String::from("LHR"), String::from("SFO")],
                ],
                vec!["JFK", "MUC", "LHR", "SFO", "SJC"],
            ),
            (
                vec![
                    vec![String::from("JFK"), String::from("SFO")],
                    vec![String::from("JFK"), String::from("ATL")],
                    vec![String::from("SFO"), String::from("ATL")],
                    vec![String::from("ATL"), String::from("JFK")],
                    vec![String::from("ATL"), String::from("SFO")],
                ],
                vec!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"],
            ),
        ];

        for (input, expected) in tests {
            assert_eq!(expected, find_itinerary(input));
        }
    }
}
