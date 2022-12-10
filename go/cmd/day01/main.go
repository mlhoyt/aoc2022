package main

import (
	"fmt"
	"sort"
	"strconv"
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
	// fmt.Println(data)

	result1 := part1(data)
	fmt.Printf("part1: %d\n", result1)

	result2 := part2(data)
	fmt.Printf("part2: %d\n", result2)
}

func parse(input string) ([][]uint32, error) {
	groups := [][]uint32{}

	for _, group := range strings.Split(input, "\n\n") {
		values := []uint32{}

		for _, line := range strings.Split(strings.TrimSpace(group), "\n") {
			value, err := strconv.Atoi(line)
			if err != nil {
				return nil, err
			}

			values = append(values, uint32(value))
		}

		groups = append(groups, values)
	}

	return groups, nil
}

func part1(data [][]uint32) uint32 {
	result := uint32(0)

	for _, group := range data {
		sum := uint32(0)

		for _, value := range group {
			sum += value
		}

		if sum > result {
			result = sum
		}
	}

	return result
}

func part2(data [][]uint32) uint32 {
	values := []uint32{}

	for _, group := range data {
		sum := uint32(0)

		for _, value := range group {
			sum += value
		}

		values = append(values, sum)
	}

	sort.Slice(values, func(i, j int) bool {
		return values[i] > values[j] // DESC
	})

	result := uint32(0)

	for _, value := range values[:3] {
		result += value
	}

	return result
}
