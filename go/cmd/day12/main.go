package main

import (
	"fmt"
	"strings"

	"github.com/mlhoyt/aoc2022/go/pkg/aoc"
)

func main() {
	input, err := aoc.ReadStdin()
	if err != nil {
		panic(err)
	}
	// fmt.Println(input)

	data, err := parse(input)
	if err != nil {
		panic(err)
	}
	// fmt.Printf("%#v\n", data)

	result1 := part1(data)
	fmt.Printf("part1: %d\n", result1)

	result2 := part2(data)
	fmt.Printf("part2: %d\n", result2)
}

func parse(input string) (data, error) {
	data := newData()
	for rn, line := range strings.Split(input, "\n") {
		data.grid = append(data.grid, []int{})

		for cn, c := range line {
			switch c {
			case 'S':
				data.start = newPoint(rn, cn)
				c = 'a'
			case 'E':
				data.end = newPoint(rn, cn)
				c = 'z'
			}

			data.grid[rn] = append(data.grid[rn], int(c-rune('a')))
		}
	}

	return data, nil
}

type data struct {
	start point
	end   point
	grid  [][]int
}

func newData() data {
	return data{
		start: newPoint(0, 0),
		end:   newPoint(0, 0),
		grid:  [][]int{},
	}
}

type point struct {
	x int
	y int
}

func newPoint(y int, x int) point {
	return point{y: y, x: x}
}

func part1(data data) uint {
	return shortestPath(data.grid, data.end, data.start)
}

func part2(data data) uint {
	allStartpoints := []point{}
	for rn := 0; rn < len(data.grid); rn++ {
		for cn := 0; cn < len(data.grid[rn]); cn++ {
			if data.grid[rn][cn] == 0 {
				allStartpoints = append(allStartpoints, newPoint(rn, cn))
			}
		}
	}

	minPathLength := uint(len(data.grid) * len(data.grid[0]))
	for _, nextStartpoint := range allStartpoints {
		nextPathLength := shortestPath(data.grid, data.end, nextStartpoint)

		if nextPathLength > 0 && nextPathLength < minPathLength {
			minPathLength = nextPathLength
		}
	}

	return minPathLength
}

func shortestPath(grid [][]int, endpoint point, startpoint point) uint {
	visited := map[point]uint{startpoint: 0}
	allPathEndpoints := []point{startpoint}

	for len(allPathEndpoints) > 0 {
		// aka popFront
		pathEndpoint := allPathEndpoints[0]
		allPathEndpoints = allPathEndpoints[1:]

		pathEndpointElevation := grid[pathEndpoint.y][pathEndpoint.x]
		pathEndpointSteps := visited[pathEndpoint]
		// fmt.Printf("endpoint=%v elevation=%d steps=%d\n", pathEndpoint, pathEndpointElevation, pathEndpointSteps)

		moves := []point{}
		for _, offset := range []point{newPoint(-1, 0), newPoint(0, 1), newPoint(1, 0), newPoint(0, -1)} {
			next := newPoint((pathEndpoint.y - offset.y), (pathEndpoint.x - offset.x))
			// below lower grid boundary
			if next.y < 0 || next.x < 0 {
				// fmt.Printf("next=%v is below lower grid boundary\n", next)
				continue
			}
			// above upper grid boundary
			if next.y >= len(grid) || next.x >= len(grid[0]) {
				// fmt.Printf("next=%v is above upper grid boundary\n", next)
				continue
			}
			// already visited
			if _, ok := visited[next]; ok {
				// fmt.Printf("next=%v is already visited\n", next)
				continue
			}
			// elevation difference too great
			nextElevation := grid[next.y][next.x]
			if nextElevation-pathEndpointElevation > 1 {
				// fmt.Printf("next=%v elevation=%d difference (to=%d) is too large\n", next, nextElevation, pathEndpointElevation)
				continue
			}

			// fmt.Printf("next=%v is valid\n", next)
			moves = append(moves, next)
		}

		for _, next := range moves {
			visited[next] = pathEndpointSteps + 1

			if next == endpoint {
				// fmt.Printf("next=%v is endpoint (steps=%d)\n", next, pathEndpointSteps+1)
				allPathEndpoints = []point{}
				break
			} else {
				allPathEndpoints = append(allPathEndpoints, next)
			}
		}
	}

	return visited[endpoint]
}
