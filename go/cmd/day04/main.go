package main

import (
	"fmt"
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

func parse(input string) ([]assignmentGroup, error) {
	groups := []assignmentGroup{}
	for _, groupStr := range strings.Split(input, "\n") {
		assignments := []assignmentRange{}
		for _, assignmentStr := range strings.Split(groupStr, ",") {
			assignment := assignmentRange{}
			for i, rangeStr := range strings.Split(assignmentStr, "-") {
				value, err := strconv.Atoi(rangeStr)
				if err != nil {
					return nil, err
				}

				if i == 0 {
					assignment.min = uint(value)
				} else {
					assignment.max = uint(value)
				}
			}

			assignments = append(assignments, assignment)
		}

		groups = append(groups, assignmentGroup{
			elf1: assignments[0],
			elf2: assignments[1],
		})
	}

	return groups, nil
}

type assignmentGroup struct {
	elf1 assignmentRange
	elf2 assignmentRange
}

type assignmentRange struct {
	min uint
	max uint
}

func (self assignmentRange) contains(other assignmentRange) bool {
	return self.min <= other.min && self.max >= other.max
}

func (self assignmentRange) overlaps(other assignmentRange) bool {
	return (self.min >= other.min && self.min <= other.max) || (self.max >= other.min && self.max <= other.max)
}

func part1(data []assignmentGroup) uint {
	count := uint(0)

	for _, group := range data {
		if group.elf1.contains(group.elf2) || group.elf2.contains(group.elf1) {
			count++
		}
	}

	return count
}

func part2(data []assignmentGroup) uint {
	count := uint(0)

	for _, group := range data {
		if group.elf1.overlaps(group.elf2) || group.elf1.contains(group.elf2) {
			count++
		}
	}

	return count
}
