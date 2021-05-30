package main

import (
	"fmt"
	"math"
	"sort"
)

type App struct {
	ID     int
	Memory int
}

func applicationPairs(deviceCapacity int, foregroundApps []App, backgroundApps []App) [][]App {
	sort.Slice(foregroundApps, func(i int, j int) bool {
		return foregroundApps[i].Memory < foregroundApps[j].Memory
	})

	sort.Slice(backgroundApps, func(i int, j int) bool {
		return backgroundApps[i].Memory > backgroundApps[j].Memory
	})

	pairs := make([][]App, 0)

	maxMemory := math.MinInt64

	left := 0

	right := len(backgroundApps) - 1

	for left < len(foregroundApps) && right >= 0 {
		memory := foregroundApps[left].Memory + backgroundApps[right].Memory

		if memory > deviceCapacity {
			right--
			continue
		}

		if memory < maxMemory {
			left++
			continue
		}

		if memory > maxMemory {
			maxMemory = memory
			pairs = make([][]App, 0)
		}

		pairs = append(pairs, []App{foregroundApps[left], backgroundApps[right]})

		i := right - 1

		for i >= 0 && backgroundApps[i].Memory == backgroundApps[i+1].Memory {
			pairs = append(pairs, []App{foregroundApps[left], backgroundApps[i]})
			i--
		}

		left++
	}

	return pairs
}

func main() {
	fmt.Println(applicationPairs(
		7,
		[]App{{ID: 1, Memory: 2}, {ID: 2, Memory: 4}, {ID: 3, Memory: 6}},
		[]App{{ID: 1, Memory: 2}}),
	)
}
