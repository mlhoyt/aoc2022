package main

import (
	"fmt"
	"math"
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

func parse(input string) (data, error) {
	ps := data{}

	for _, line := range strings.Split(input, "\n") {
		fields := strings.Split(line, " ")

		if len(fields) != 2 {
			return nil, fmt.Errorf("cannot parse line: too few fields; expected 2 but found %d", len(fields))
		}

		dir := point{}
		switch fields[0] {
		case "U":
			dir = point{x: 0, y: 1}
		case "R":
			dir = point{x: 1, y: 0}
		case "D":
			dir = point{x: 0, y: -1}
		case "L":
			dir = point{x: -1, y: 0}
		default:
			return nil, fmt.Errorf("cannot parse direction: %s", fields[0])
		}

		dist, err := strconv.Atoi(fields[1])
		if err != nil {
			return nil, err
		}

		for i := 0; i < dist; i++ {
			ps = append(ps, dir)
		}
	}

	return ps, nil
}

type data = []point

type point struct {
	x int
	y int
}

func (self point) add(other point) point {
	return point{
		x: self.x + other.x,
		y: self.y + other.y,
	}
}

func (self point) follow(other point) point {
	dx := other.x - self.x
	dy := other.y - self.y

	xp := self.x
	if dx != 0 {
		xp = self.x + (dx / abs(dx))
	}

	yp := self.y
	if dy != 0 {
		yp = self.y + (dy / abs(dy))
	}

	if abs(dx) > 1 || abs(dy) > 1 {
		return point{x: xp, y: yp}
	}

	return self
}

// TODO: This should be generic on any of the int types
func abs(v int) int {
	return int(math.Abs(float64(v)))
}

func part1(data data) uint {
	head := point{}
	tail := point{}
	ps := aoc.NewHashSet[point]()

	for _, cmd := range data {
		head = head.add(cmd)
		tail = tail.follow(head)

		ps.Insert(tail)
	}

	return ps.Length()
}

func part2(data data) uint {
	chain := [10]point{}
	ps := aoc.NewHashSet[point]()

	for _, cmd := range data {
		chain[0] = chain[0].add(cmd)
		for i := 1; i < len(chain); i++ {
			chain[i] = chain[i].follow(chain[i-1])
		}

		ps.Insert(chain[len(chain)-1])
	}

	return ps.Length()
}
