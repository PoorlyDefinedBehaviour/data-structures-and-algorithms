package welshpowell

import (
	"sort"
)

type (
	Graph           map[string][]string
	ColoredVertices map[string]int
)

func (coloredVertices ColoredVertices) isColored(vertex string) bool {
	_, colored := coloredVertices[vertex]

	return colored
}

func (graph Graph) vertices() []string {
	vertices := make([]string, 0, len(graph))

	for key := range graph {
		vertices = append(vertices, key)
	}

	return vertices
}

func (graph Graph) neighbors(vertex string) []string {
	return graph[vertex]
}

func (graph Graph) isConnectedToVertexWithColor(coloredVertices ColoredVertices, color int, vertex string) bool {
	for _, neighbor := range graph.neighbors(vertex) {
		neighborColor, isColored := coloredVertices[neighbor]

		if isColored && neighborColor == color {
			return true
		}
	}

	return false
}

func (graph Graph) degree(vertex string) int {
	neighbors := graph[vertex]

	return len(neighbors)
}

func sortByDegree(graph Graph, vertices []string) []string {
	sortedVertices := make([]string, len(vertices))

	copy(sortedVertices, vertices)

	sort.Slice(sortedVertices, func(i int, j int) bool {
		return graph.degree(vertices[i]) > graph.degree(vertices[j])
	})

	return sortedVertices
}

func WelshPowell(graph Graph) ColoredVertices {
	coloredVertices := make(ColoredVertices, len(graph))

	vertices := sortByDegree(graph, graph.vertices())

	color := 0

	for index, vertex := range vertices {
		if coloredVertices.isColored(vertex) {
			continue
		}

		coloredVertices[vertex] = color

		for i := index + 1; i < len(vertices); i++ {
			otherVertex := vertices[i]

			if coloredVertices.isColored(otherVertex) || graph.isConnectedToVertexWithColor(coloredVertices, color, otherVertex) {
				continue
			}

			coloredVertices[otherVertex] = color
		}

		color++
	}

	return coloredVertices
}
