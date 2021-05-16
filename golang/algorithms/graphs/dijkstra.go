package main

import (
	"container/heap"
	"fmt"
	"math"
)

type Item struct {
	vertex   Vertex
	distance int
}

type PriorityQueue []*Item

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].distance < pq[j].distance
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PriorityQueue) Push(x interface{}) {
	item := x.(*Item)
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

type Vertex int

type Edge struct {
	to   Vertex
	cost int
}

type Graph map[Vertex][]Edge

const (
	Infinity    = math.MaxInt64
	Unitialized = -1
)

func (graph *Graph) vertices() []Vertex {
	vertices := make([]Vertex, 0, len(*graph))

	for vertex := range *graph {
		vertices = append(vertices, vertex)
	}

	return vertices
}

func (graph *Graph) edges(vertex Vertex) []Edge {
	return (*graph)[vertex]
}

func dijkstra(graph *Graph, startingVertex Vertex) (map[Vertex]int, map[Vertex]Vertex) {
	distances := make(map[Vertex]int)
	previous := make(map[Vertex]Vertex)
	visited := make(map[Vertex]bool, len(*graph))
	priorityQueue := PriorityQueue{}

	for _, vertex := range graph.vertices() {
		distances[vertex] = Infinity
		previous[vertex] = Unitialized
	}

	distances[startingVertex] = 0

	heap.Push(&priorityQueue, &Item{
		vertex:   startingVertex,
		distance: 0,
	})

	for len(priorityQueue) != 0 {
		item := heap.Pop(&priorityQueue).(*Item)
		vertex := item.vertex

		visited[vertex] = true

		for _, edge := range graph.edges(vertex) {
			if visited[edge.to] {
				continue
			}

			newDistance := distances[vertex] + edge.cost

			if newDistance < distances[edge.to] {
				previous[edge.to] = vertex
				distances[edge.to] = newDistance

				heap.Push(&priorityQueue, &Item{vertex: edge.to, distance: distances[edge.to]})
			}
		}
	}

	return distances, previous
}

func main() {
	graph := Graph{
		0: []Edge{{to: 1, cost: 3}, {to: 2, cost: 6}},
		1: []Edge{{to: 0, cost: 3}, {to: 2, cost: 2}, {to: 3, cost: 1}},
		2: []Edge{{to: 1, cost: 6}, {to: 1, cost: 2}, {to: 3, cost: 1}, {to: 4, cost: 4}, {to: 5, cost: 2}},
		3: []Edge{{to: 1, cost: 1}, {to: 2, cost: 1}, {to: 4, cost: 2}, {to: 6, cost: 4}},
		4: []Edge{{to: 2, cost: 4}, {to: 3, cost: 2}, {to: 5, cost: 2}, {to: 6, cost: 1}},
		5: []Edge{{to: 2, cost: 2}, {to: 4, cost: 2}, {to: 6, cost: 1}},
		6: []Edge{{to: 3, cost: 4}, {to: 4, cost: 1}, {to: 5, cost: 1}},
	}

	fmt.Println(dijkstra(&graph, 0))
}
