package main

/*
You have a graph of n nodes. You are given an integer n and
an array edges where edges[i] = [ai, bi] indiciates that
there is an edge between ai and b1 in the graph.

Return the number of connected components in the graph.

Examples:

Input: n=5, edges = [[0,1], [1,2], [3,4]]
Output: 2
*/

func edgesToAdjacencyList(edges [][]int) map[int][]int {
	adjacencyList := make(map[int][]int)

	for _, edge := range edges {
		from := edge[0]
		to := edge[1]

		adjacencyList[from] = append(adjacencyList[from], to)
		adjacencyList[to] = append(adjacencyList[to], from)
	}

	return adjacencyList
}

func dfs(startingVertex int, adjancecyList map[int][]int, visited map[int]bool) {
	if visited[startingVertex] {
		return
	}

	visited[startingVertex] = true

	for _, neighbor := range adjancecyList[startingVertex] {
		dfs(neighbor, adjancecyList, visited)
	}
}

// time O(E + V)
// space O(V)
func countComponentsUsingDfs(edges [][]int) int {
	adjacencyList := edgesToAdjacencyList(edges)

	visited := make(map[int]bool)

	components := 0

	for vertex := range adjacencyList {
		if visited[vertex] {
			continue
		}

		components++

		dfs(vertex, adjacencyList, visited)
	}

	return components
}

func main() {
}
