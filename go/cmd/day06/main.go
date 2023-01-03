package main

import (
	"fmt"

	"github.com/mlhoyt/aoc2022/go/pkg/aoc"
)

func main() {
	input, err := aoc.ReadStdin()
	if err != nil {
		panic(err)
	}
	// fmt.Println(input)

	result1 := part1(input)
	fmt.Printf("part1: %d\n", result1)

	result2 := part2(input)
	fmt.Printf("part2: %d\n", result2)
}

func part1(input string) uint {
	const WINDOW_SIZE = 4

	for i, w := range sliceToWindows([]rune(input), WINDOW_SIZE) {
		set := aoc.NewHashSetFromSlice(w)
		if len(set) == WINDOW_SIZE {
			return uint(i) + WINDOW_SIZE
		}
	}

	return 0
}

func part2(input string) uint {
	const WINDOW_SIZE = 14

	for i, w := range sliceToWindows([]rune(input), WINDOW_SIZE) {
		set := aoc.NewHashSetFromSlice(w)
		if len(set) == WINDOW_SIZE {
			return uint(i) + WINDOW_SIZE
		}
	}

	return 0
}

func sliceToWindows[T any](vs []T, n uint) [][]T {
	if n < 1 || n > uint(len(vs)) {
		return nil
	}

	ws := [][]T{}

	for i := 0; i <= len(vs)-int(n); i++ {
		w := vs[i : i+int(n)]
		ws = append(ws, w)
	}

	return ws
}
