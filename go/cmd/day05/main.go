package main

import (
	"errors"
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
	fmt.Printf("part1: %s\n", result1)

	result2 := part2(data)
	fmt.Printf("part2: %s\n", result2)
}

func parse(input string) (data, error) {
	data := data{}

	lines := strings.Split(input, "\n")
	linesIdx := 0

	// Process stack definitions
	for ; linesIdx < len(lines); linesIdx++ {
		line := lines[linesIdx]

		if strings.HasPrefix(line, " 1 ") {
			linesIdx++
			break
		}

		lineOffset := 0
		stackIndex := 0
		for lineOffset < len(line) {
			chunk := line[lineOffset:]

			if len(chunk) < 3 {
				break
			}

			// Initialize stack if necessary
			if len(data.stacks) < (stackIndex + 1) {
				data.stacks = append(data.stacks, stack{})
			}

			// Push to stack
			if strings.HasPrefix(chunk, "[") {
				data.stacks[stackIndex] = append(data.stacks[stackIndex], chunk[1])
			}

			// Update line-offset and queue index
			lineOffset += 4
			stackIndex++
		}
	}

	// Process empty separator line
	if linesIdx < len(lines) {
		if strings.TrimSpace(lines[linesIdx]) != "" {
			return data, errors.New("parsing failed expecting a blank line")
		}
	} else {
		return data, errors.New("parsing failed expecting a blank line")
	}

	linesIdx++

	// Process instructions
	for ; linesIdx < len(lines); linesIdx++ {
		line := lines[linesIdx]

		fields := strings.Split(line, " ")

		if len(fields) != 6 {
			return data, errors.New("parsing instruction yielded too few fields")
		}

		if fields[0] != "move" {
			return data, errors.New("parsing instruction yielded no move field")
		}

		count, err := strconv.Atoi(fields[1])
		if err != nil {
			return data, err
		}

		if fields[2] != "from" {
			return data, errors.New("parsing instruction yielded no from field")
		}

		from, err := strconv.Atoi(fields[3])
		if err != nil {
			return data, err
		}

		if fields[4] != "to" {
			return data, errors.New("parsing instruction yielded no to field")
		}

		to, err := strconv.Atoi(fields[5])
		if err != nil {
			return data, err
		}

		// Push to instruction list
		data.instructions = append(data.instructions, instruction{
			count: uint(count),
			from:  uint(from - 1),
			to:    uint(to - 1),
		})
	}

	return data, nil
}

type data struct {
	stacks       []stack
	instructions []instruction
}

type stack = []byte

type instruction struct {
	count uint
	from  uint
	to    uint
}

func part1(data data) string {
	// Deep copy
	stacks := append([]stack{}, data.stacks...)

	for _, instruction := range data.instructions {
		stacks = applyInstruction9000(stacks, instruction)
	}

	code := ""
	for _, stack := range stacks {
		code += string(stack[0])
	}

	return code
}

func applyInstruction9000(data []stack, instruction instruction) []stack {
	// Shallow copy... but that is okay
	nextData := data

	// Move one container at a time from front to front.
	for i := uint(0); i < instruction.count; i++ {
		nextData[instruction.to] = append(stack{nextData[instruction.from][0]}, nextData[instruction.to]...)
		nextData[instruction.from] = nextData[instruction.from][1:]
	}

	return nextData
}

func part2(data data) string {
	// Deep copy
	stacks := append([]stack{}, data.stacks...)

	for _, instruction := range data.instructions {
		stacks = applyInstruction9001(stacks, instruction)
	}

	code := ""
	for _, stack := range stacks {
		code += string(stack[0])
	}

	return code
}

func applyInstruction9001(data []stack, instruction instruction) []stack {
	// Shallow copy... but that is okay
	nextData := data

	// Move count containers at a time from front to front.
	// This deep copy is critical and took far too long to debug as the crux.
	moving := append(stack{}, nextData[instruction.from][:instruction.count]...)
	nextData[instruction.to] = append(moving, nextData[instruction.to]...)
	nextData[instruction.from] = nextData[instruction.from][instruction.count:]

	return nextData
}
