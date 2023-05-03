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
	fmt.Println(input)

	data, err := parse(input)
	if err != nil {
		panic(err)
	}
	fmt.Println(data)

	sourcePoint := point2D{x: 500, y: 0}

	bottom := uint(0)
	for p := range data {
		if p.y > bottom {
			bottom = p.y
		}
	}

	term1Fn := func(p point2D) bool { return p.y >= bottom }
	term2Fn := func(p point2D) bool { return p.y >= (bottom + 2) }
	noopFn := func(p point2D) bool { return false }

	result1 := simulate(data, sourcePoint, noopFn, term1Fn)
	fmt.Printf("part1: %d\n", result1)

	result2 := simulate(data, sourcePoint, term2Fn, noopFn)
	fmt.Printf("part2: %d\n", result2)
}

func parse(input string) (data, error) {
	data := data{}

	for _, line := range strings.Split(input, "\n") {
		// Extract each "x,y" point and convert to a point2D{x,y}
		points := []point2D{}
		for _, point := range strings.Split(line, " -> ") {
			vs := strings.Split(point, ",")

			x, err := strconv.Atoi(vs[0])
			if err != nil {
				return data, err
			}

			y, err := strconv.Atoi(vs[1])
			if err != nil {
				return data, err
			}

			points = append(points, point2D{x: uint(x), y: uint(y)})
		}

		// Convert each pair of adjacent points (a line) into each individual point and store.
		for i := 0; i < len(points)-1; i++ {
			p1 := points[i]
			p2 := points[i+1]

			if p1.x == p2.x {
				ys := []uint{}
				if p1.y < p2.y {
					for yi := p1.y; yi <= p2.y; yi++ {
						ys = append(ys, yi)
					}
				} else {
					for yi := p2.y; yi <= p1.y; yi++ {
						ys = append(ys, yi)
					}
				}

				for _, y := range ys {
					data[point2D{x: p1.x, y: y}] = pointTypeRock{}
				}
			} else {
				xs := []uint{}
				if p1.x < p2.x {
					for xi := p1.x; xi <= p2.x; xi++ {
						xs = append(xs, xi)
					}
				} else {
					for xi := p2.x; xi <= p1.x; xi++ {
						xs = append(xs, xi)
					}
				}

				for _, x := range xs {
					data[point2D{x: x, y: p1.y}] = pointTypeRock{}
				}
			}
		}
	}

	return data, nil
}

type data = map[point2D]pointType

type point2D struct {
	x uint
	y uint
}

type pointType interface {
	isPointType()
}

type pointTypeRock struct{}

func (self pointTypeRock) isPointType() {}

func (self pointTypeRock) String() string {
	return "#"
}

type pointTypeSand struct{}

func (self pointTypeSand) isPointType() {}

func (self pointTypeSand) String() string {
	return "o"
}

func simulate(dataRef data, startPoint point2D, floorFn func(point2D) bool, termFn func(point2D) bool) uint {
	// Clone dataRef
	data := data{}
	for p, pt := range dataRef {
		data[p] = pt
	}

	// Keep dropping units of sand until one of the two termination points is reached
	for {
		// showState(data)

		nextPoint := step(data, startPoint, floorFn, termFn)
		if nextPoint != nil {
			data[*nextPoint] = pointTypeSand{}

			// part2 termination (unable to move)
			if *nextPoint == startPoint {
				break
			}
		} else {
			// part1 termination (overflow beyond the bottom)
			break
		}
	}

	// Count the units of sand
	count := uint(0)
	for _, pt := range data {
		switch pt.(type) {
		case pointTypeSand:
			count++
		}
	}

	return count
}

// A unit of sand always falls down one step if possible.
// If the tile immediately below is blocked then the sand attempts to move diagonally one step down
// and to the left.
// If the tile down and to the left is blocked then the sand attempts to move diagonally one step down and to the right.
// If all three destinations are blocked then the sand comes to rest.
// If a unit of sand can move to a destination below the bottom then stop.
func step(data data, startPoint point2D, floorFn func(point2D) bool, termFn func(point2D) bool) *point2D {
	currPoint := startPoint

	for {
		moves := []point2D{
			{x: currPoint.x, y: currPoint.y + 1},
			{x: currPoint.x - 1, y: currPoint.y + 1},
			{x: currPoint.x + 1, y: currPoint.y + 1},
		}

		validMoves := []point2D{}
		for _, move := range moves {
			if _, ok := data[move]; !ok && !floorFn(move) {
				validMoves = append(validMoves, move)
			}
		}

		// at rest
		if len(validMoves) == 0 {
			return &currPoint
		}

		// moved
		currPoint = validMoves[0]

		if termFn(currPoint) {
			break
		}
	}

	return nil
}

func showState(data data) {
	xMin := uint(math.MaxUint)
	xMax := uint(0)
	yMax := uint(0)
	for p := range data {
		if p.x < xMin {
			xMin = p.x
		}
		if p.x > xMax {
			xMax = p.x
		}
		if p.y > yMax {
			yMax = p.y
		}
	}

	for y := uint(0); y <= yMax+1; y++ {
		for x := xMin - 1; x <= xMax+1; x++ {
			if pt, ok := data[point2D{x, y}]; ok {
				fmt.Printf("%s", pt)
			} else {
				fmt.Printf(".")
			}
		}
		fmt.Println()
	}
}
