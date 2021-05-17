package main

import (
	"container/heap"
	"fmt"
	"math"
)

type Item struct {
	vertex Vertex
	fCost  float32
}

type PriorityQueue []*Item

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].fCost < pq[j].fCost
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PriorityQueue) Push(x interface{}) {
	item, ok := x.(*Item)
	if !ok {
		panic("pq.Push: argument is not a *Item")
	}

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

type Vertex struct {
	value     string
	latitude  float32
	longitude float32
}

type Graph map[Vertex][]Vertex

var none = Vertex{
	value:     "none",
	latitude:  -1.0,
	longitude: -1.0,
}

func (graph Graph) vertices() []Vertex {
	vertices := make([]Vertex, 0, len(graph))

	for vertex := range graph {
		vertices = append(vertices, vertex)
	}

	return vertices
}

func (graph Graph) neighbors(vertex Vertex) []Vertex {
	return (graph)[vertex]
}

func manhattanDistance(x1 float32, y1 float32, x2 float32, y2 float32) float32 {
	return float32(math.Abs(float64(x1-x2)) + math.Abs(float64(y1-y2)))
}

func (graph Graph) distance(from Vertex, to Vertex) float32 {
	return manhattanDistance(from.latitude, from.longitude, to.latitude, to.longitude)
}

// the fact that go does not have a function to reverse a slice LMAO.
func reverse(xs []Vertex) []Vertex {
	ys := make([]Vertex, 0, len(xs))

	for i := len(xs) - 1; i > -1; i-- {
		ys = append(ys, xs[i])
	}

	return ys
}

func buildPath(pathMap map[Vertex]Vertex, from Vertex, to Vertex) []Vertex {
	path := []Vertex{}

	current := to

	for current != from {
		path = append(path, current)
		current = pathMap[current]
	}

	path = append(path, from)

	return reverse(path)
}

func aStar(graph Graph, from Vertex, to Vertex) []Vertex {
	priorityQueue := PriorityQueue{}
	visited := make(map[Vertex]bool)
	previous := make(map[Vertex]Vertex)

	for _, vertex := range graph.vertices() {
		previous[vertex] = none
	}

	// G cost = distance from starting node
	// H cost(heuristic) = distance from end node
	// F cost = G cost + H Cost

	heap.Push(&priorityQueue, &Item{
		vertex: from,
		fCost:  graph.distance(from, to),
	})

	for len(priorityQueue) != 0 {
		//nolint:forcetypeassert
		item := heap.Pop(&priorityQueue).(*Item)

		if item.vertex == to {
			break
		}

		visited[item.vertex] = true

		for _, edge := range graph.neighbors(item.vertex) {
			if !visited[edge] {
				heap.Push(&priorityQueue, &Item{
					vertex: edge,
					fCost:  graph.distance(from, edge) + graph.distance(edge, to),
				})

				previous[edge] = item.vertex
			}
		}
	}

	return buildPath(previous, from, to)
}

func main() {
	cities := map[string]Vertex{
		"Cascavel":          {value: "Cascavel", latitude: 24.9578, longitude: 53.4595},
		"Foz do Iguaçu":     {value: "Foz do Iguaçu", latitude: 25.5163, longitude: 54.5854},
		"Toledo":            {value: "Toledo", latitude: 24.7251, longitude: 53.7417},
		"Francisco Beltrão": {value: "Francisco Beltrão", latitude: 26.0779, longitude: 53.0520},
		"São Mateus do Sul": {value: "São Mateus do Sul", latitude: 25.8682, longitude: 50.3842},
		"Curitiba":          {value: "Curitiba", latitude: 25.4290, longitude: 49.2671},
		"Paranaguá":         {value: "Paranaguá", latitude: 25.5149, longitude: 48.5226},
		"Ponta Grossa":      {value: "Ponta Grossa", latitude: 25.0994, longitude: 50.1583},
		"Guarapuava":        {value: "Guarapuava", latitude: 25.3907, longitude: 51.4628},
		"Maringá":           {value: "Maringá", latitude: 23.4210, longitude: 51.9331},
		"Umuarama":          {value: "Umuarama", latitude: 23.7661, longitude: 53.3206},
		"Londrina":          {value: "Londrina", latitude: 23.3045, longitude: 51.1696},
	}

	graph := map[Vertex][]Vertex{
		cities["Cascavel"]:          {cities["Toledo"], cities["Foz do Iguaçu"], cities["Francisco Beltrão"], cities["Guarapuava"]},
		cities["Foz do Iguaçu"]:     {cities["Cascavel"]},
		cities["Toledo"]:            {cities["Cascavel"], cities["Umuarama"]},
		cities["Francisco Beltrão"]: {cities["Cascavel"], cities["São Mateus do Sul"]},
		cities["São Mateus do Sul"]: {cities["Francisco Beltrão"], cities["Curitiba"]},
		cities["Curitiba"]:          {cities["São Mateus do Sul"], cities["Ponta Grossa"], cities["Paranaguá"]},
		cities["Paranaguá"]:         {cities["Curitiba"]},
		cities["Ponta Grossa"]:      {cities["Curitiba"], cities["Guarapuava"], cities["Maringá"], cities["Londrina"]},
		cities["Guarapuava"]:        {cities["Cascavel"], cities["Ponta Grossa"]},
		cities["Maringá"]:           {cities["Ponta Grossa"], cities["Umuarama"], cities["Londrina"]},
		cities["Umuarama"]:          {cities["Toledo"], cities["Maringá"]},
		cities["Londrina"]:          {cities["Maringá"], cities["Ponta Grossa"]},
	}

	fmt.Println(aStar(graph, cities["Cascavel"], cities["Curitiba"]))
}
