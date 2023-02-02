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

	states := simulate(data)
	// fmt.Println("states")
	// for i, v := range states {
	// 	fmt.Printf("cycle=%d reg_x=%d\n", i, v)
	// }

	result1 := part1(states)
	fmt.Printf("part1: %d\n", result1)

	result2 := part2(states)
	fmt.Printf("part2: %d\n", result2)
}

func parse(input string) (data, error) {
	data := []instruction{}

	for _, line := range strings.Split(input, "\n") {
		fields := strings.Split(line, " ")

		if len(fields) < 1 {
			return nil, fmt.Errorf("unexpected non-instruction input")
		}

		switch fields[0] {
		case "addx":
			if len(fields) != 2 {
				return nil, fmt.Errorf("addx instruction requires an unsigned integer argument")
			}

			value, err := strconv.Atoi(fields[1])
			if err != nil {
				return nil, err
			}

			data = append(data, instructionAddx{value: value})
		case "noop":
			data = append(data, instructionNoop{})
		default:
			return nil, fmt.Errorf("unexpected instruction: %s", line)
		}
	}

	return data, nil
}

type data = []instruction

// instruction is an algebraic type with variants: instructionAddx, instructionNoop
type instruction interface {
	isInstruction()
}

type instructionAddx struct {
	value int
}

func (instructionAddx) isInstruction() {}

func (self instructionAddx) getValue() int {
	return self.value
}

type instructionNoop struct{}

func (instructionNoop) isInstruction() {}

func simulate(data data) []int {
	states := []int{1, 1}

	for _, i := range data {
		curr := states[len(states)-1]

		switch it := i.(type) {
		case instructionAddx:
			states = append(states, curr, curr+it.getValue())
		case instructionNoop:
			states = append(states, curr)
		}
	}

	return states
}

func part1(states []int) uint {
	sum := 0
	for i, v := range states {
		if (i-20)%40 == 0 {
			sum += (i * v)
		}
	}

	return uint(sum)
}

func part2(states []int) uint {
	output := []rune{}

	for i, v := range states {
		if i > 0 {
			x := int((i - 1) % 40)
			if x >= v-1 && x <= v+1 {
				output = append(output, '#')
			} else {
				output = append(output, '.')
			}
		}
	}

	for i := 0; i < 240; i += 40 {
		fmt.Printf("%s\n", string(output[i:i+40]))
	}

	return 0
}
