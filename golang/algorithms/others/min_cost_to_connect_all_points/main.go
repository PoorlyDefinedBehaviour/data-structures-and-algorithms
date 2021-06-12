package main

import (
	"container/heap"
	"math"
)

/*
You are given an array points representing integer coordinates of some points on a 2D-plane,
where points[i] = [xi, yi].

The cost of connecting two points [xi, yi] and [xj, yj] is the manhattan distance between them:
|xi - xj| + |yi - yj|, where |val| denotes the absolute value of val.

Return the minimum cost to make all points connected.
All points are connected if there is exactly one simple path between any two points.

Example 1:

Input: points = [[0,0],[2,2],[3,10],[5,2],[7,0]]
Output: 20
Explanation:

We can connect the points as shown above to get the minimum cost of 20.
Notice that there is a unique path between every pair of points.

Example 2:

Input: points = [[3,12],[-2,5],[-4,1]]
Output: 18

Example 3:

Input: points = [[0,0],[1,1],[1,0],[-1,1]]
Output: 4

Example 4:

Input: points = [[-1000000,-1000000],[1000000,1000000]]
Output: 4000000

Example 5:

Input: points = [[0,0]]
Output: 0

Constraints:

    1 <= points.length <= 1000
    -106 <= xi, yi <= 106
    All pairs (xi, yi) are distinct.
*/

type Point struct {
	X int
	Y int
}

type Edge struct {
	Point Point
	Cost  int
}

type PriorityQueue []*Edge

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].Cost < pq[j].Cost
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PriorityQueue) Push(x interface{}) {
	item := x.(*Edge)
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil
	*pq = old[0 : n-1]
	return item
}

func manhattanDistance(from, to Point) int {
	return int(math.Abs(float64(from.X-to.X))) + int(math.Abs(float64(from.Y-to.Y)))
}

// time O(n log n)
// where n is the number of points
// space O(n log n)
func minCostConnectPoints(points [][]int) int {
	if len(points) == 0 {
		return 0
	}

	startingPoint := &Edge{
		Point: Point{X: points[0][0], Y: points[0][1]},
		Cost:  0,
	}

	priorityQueue := PriorityQueue{startingPoint}

	cost := 0

	visitedPoints := make(map[Point]bool)

	for len(priorityQueue) > 0 && len(visitedPoints) < len(points) {
		edge := heap.Pop(&priorityQueue).(*Edge)

		if visitedPoints[edge.Point] {
			continue
		}

		visitedPoints[edge.Point] = true

		cost += edge.Cost

		for _, point := range points {
			p := Point{X: point[0], Y: point[1]}

			if visitedPoints[p] {
				continue
			}

			heap.Push(&priorityQueue, &Edge{
				Point: p,
				Cost:  manhattanDistance(edge.Point, p),
			})
		}
	}

	return cost
}

func main() {

}
