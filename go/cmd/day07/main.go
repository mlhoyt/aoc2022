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

	data := parse(input)
	// fmt.Printf("%v\n", data)

	result1 := part1(data)
	fmt.Printf("part1: %d\n", result1)

	result2 := part2(data)
	fmt.Printf("part2: %d\n", result2)
}

func parse(input string) data {
	pwd := []string{""}
	data := data{}

	pwdToDirs := func(ss []string) [][]string {
		dirs := [][]string{}

		for i := 1; i <= len(ss); i++ {
			dirs = append(dirs, ss[:i])
		}

		return dirs
	}

	for _, line := range strings.Split(input, "\n") {
		if strings.HasPrefix(line, "$ cd ") {
			cmd := strings.Split(line, " ")
			dir := cmd[len(cmd)-1]

			if dir == ".." {
				pwd = pwd[:len(pwd)-1]
			} else if dir == "/" {
				pwd = []string{""}
			} else {
				pwd = append(pwd, dir)
			}
		} else if strings.HasPrefix(line, "$ ls") {
			// do nothing
		} else if strings.HasPrefix(line, "dir ") {
			// do nothing
		} else {
			cmd := strings.Split(line, " ")
			fileSize, err := strconv.Atoi(cmd[0])
			if err != nil {
				panic(err)
			}

			for _, dir := range pwdToDirs(pwd) {
				path := strings.Join(dir, "/")

				if _, ok := data[path]; !ok {
					data[path] = 0
				}

				data[path] += uint(fileSize)
			}
		}
	}

	return data
}

type data = map[string]uint

func part1(data data) uint {
	sum := uint(0)
	for _, v := range data {
		if v <= 100_000 {
			sum += v
		}
	}

	return sum
}

func part2(data data) uint {
	used := data[""]
	free := 70_000_000 - used
	toFree := 30_000_000 - free

	minSizeToFree := uint(70_000_000)
	for _, s := range data {
		if s < minSizeToFree && s > toFree {
			minSizeToFree = s
		}
	}

	return minSizeToFree
}
