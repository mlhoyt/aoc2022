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

func parse(input string) (data, error) {
	data := data{}

	for _, l := range strings.Split(input, "\n") {
		row := []uint{}
		for _, c := range l {
			value, err := strconv.Atoi(string(c))
			if err != nil {
				return nil, err
			}

			row = append(row, uint(value))
		}

		data = append(data, row)
	}

	return data, nil
}

type data = [][]uint

type point struct {
	row uint
	col uint
}

func part1(data data) uint {
	canBeSeen := aoc.HashSet[point]{}

	for r := range data {
		for c := range data[r] {
			cbsKey := point{row: uint(r), col: uint(c)}

			if r == 0 || r == len(data)-1 || c == 0 || c == len(data[r])-1 {
				canBeSeen.Insert(cbsKey)
			} else {
				// from north
				cbsn := true
				for rp := r - 1; rp >= 0; rp-- {
					if data[r][c] <= data[rp][c] {
						cbsn = false
					}
				}
				if cbsn {
					canBeSeen.Insert(cbsKey)
				}

				// from east
				cbse := true
				for cp := c + 1; cp < len(data[r]); cp++ {
					if data[r][c] <= data[r][cp] {
						cbse = false
					}
				}
				if cbse {
					canBeSeen.Insert(cbsKey)
				}

				// from south
				cbss := true
				for rp := r + 1; rp < len(data); rp++ {
					if data[r][c] <= data[rp][c] {
						cbss = false
					}
				}
				if cbss {
					canBeSeen.Insert(cbsKey)
				}

				// from west
				cbsw := true
				for cp := c - 1; cp >= 0; cp-- {
					if data[r][c] <= data[r][cp] {
						cbsw = false
					}
				}
				if cbsw {
					canBeSeen.Insert(cbsKey)
				}
			}
		}
	}

	return uint(len(canBeSeen))
}

func part2(data data) uint {
	scenicScores := map[point]uint{}

	for r := range data {
		for c := range data[r] {
			ssKey := point{row: uint(r), col: uint(c)}

			if r == 0 || r == len(data)-1 || c == 0 || c == len(data[r])-1 {
				scenicScores[ssKey] = uint(0)
			} else {
				// looking north
				ssn, isBlocked := 0, false
				for rp := r - 1; rp >= 0; rp-- {
					if !isBlocked {
						ssn++
						isBlocked = data[rp][c] >= data[r][c]
					}
				}

				// looking east
				sse, isBlocked := 0, false
				for cp := c + 1; cp < len(data[r]); cp++ {
					if !isBlocked {
						sse++
						isBlocked = data[r][cp] >= data[r][c]
					}
				}

				// looking south
				sss, isBlocked := 0, false
				for rp := r + 1; rp < len(data); rp++ {
					if !isBlocked {
						sss++
						isBlocked = data[rp][c] >= data[r][c]
					}
				}

				// looking west
				ssw, isBlocked := 0, false
				for cp := c - 1; cp >= 0; cp-- {
					if !isBlocked {
						ssw++
						isBlocked = data[r][cp] >= data[r][c]
					}
				}

				scenicScores[ssKey] = uint(ssn * sse * sss * ssw)
			}
		}
	}

	maxScenicScore := uint(0)
	for _, v := range scenicScores {
		if v > maxScenicScore {
			maxScenicScore = v
		}
	}

	return maxScenicScore
}
