package main

import (
	"fmt"
	"math"
	"sort"
)

type edge struct {
	from int
	to   int
	cost int
}

func buildOptimalRouteFromSavings(savings []edge, costMatrix [][]int) []edge {
	sort.Slice(savings, func(i, j int) bool { return savings[i].cost > savings[j].cost })

	route := make([]edge, 0)

	seen := make(map[int]bool)

	for _, edge := range savings {
		if !seen[edge.from] {
			route = append(route, edge)
		}

		seen[edge.from] = true
	}

	sort.Slice(route, func(i, j int) bool { return route[i].from < route[j].from })

	lastStoppingPointBeforeReturning := route[len(route)-1]

	route = append(route, edge{
		from: lastStoppingPointBeforeReturning.to,
		to:   route[0].from,
		cost: costMatrix[lastStoppingPointBeforeReturning.to][0],
	})

	return route
}

func calculateSavings(costMatrix [][]int) []edge {
	savings := make([]edge, 0, len(costMatrix))

	for i := 0; i < len(costMatrix); i++ {
		for j := i + 1; j < len(costMatrix); j++ {
			savings = append(savings, edge{
				from: i,
				to:   j,
				cost: costMatrix[i][0] + costMatrix[0][j] - costMatrix[i][j],
			})
		}
	}

	return savings
}

func calculatePathThroughSavingsMethod(costMatrix [][]int) []edge {
	// Savings[i][j] = Costs[i][0] + Costs[0][j] - Costs[i][j]
	//
	// Select any city as the starting point.
	// Assume every stopping point can be reached by going to
	// the point and coming back to the starting point.
	// Calculate all savings for joining points and removing
	// a trip back to the starting point.
	// Order savings from greatest to smallest.
	// Form route based on savings and try to save the
	// maximum amount possible.
	savings := calculateSavings(costMatrix)

	return buildOptimalRouteFromSavings(savings, costMatrix)
}

func main() {
	graph := [][]int{
		{math.MaxInt64, 8, 9, 13, 10},
		{8, math.MaxInt64, 4, 11, 13},
		{9, 4, math.MaxInt64, 5, 8},
		{13, 11, 5, math.MaxInt64, 7},
		{10, 13, 8, 7, math.MaxInt64},
	}

	fmt.Printf("\n\naaaaaaa calculatePathThroughSavingsMethod(graph) %+v\n\n", calculatePathThroughSavingsMethod(graph))
}
