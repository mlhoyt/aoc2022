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

	data := parse(input)
	// fmt.Println(data)

	result1 := part1(data)
	fmt.Println("part1: ", result1)

	result2 := part2(data)
	fmt.Println("part2: ", result2)
}

func parse(input string) []rucksak {
	rucksaks := []rucksak{}

	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		rucksaks = append(rucksaks, rucksak{
			p1: newHashMapFromSlice([]byte(line[:len(line)/2])),
			p2: newHashMapFromSlice([]byte(line[len(line)/2:])),
		})
	}

	return rucksaks
}

type rucksak struct {
	p1 hashMap[byte, uint]
	p2 hashMap[byte, uint]
}

type hashMap[K comparable, V any] map[K]V

func newHashMapFromSlice[K comparable](vs []K) hashMap[K, uint] {
	hashMap := hashMap[K, uint]{}

	for _, v := range vs {
		if hashMap.contains(v) {
			hashMap[v] = hashMap[v] + 1
		} else {
			hashMap[v] = 1
		}
	}

	return hashMap
}

func (self hashMap[K, V]) contains(v K) bool {
	_, ok := self[v]

	return ok
}

func (self hashMap[K, V]) keys() []K {
	keys := []K{}

	for k := range self {
		keys = append(keys, k)
	}

	return keys
}

type hashSet[K comparable] map[K]struct{}

func newHashSetFromSlice[K comparable](vs []K) hashSet[K] {
	hashSet := hashSet[K]{}

	for _, v := range vs {
		hashSet[v] = struct{}{}
	}

	return hashSet
}

func (self hashSet[K]) contains(v K) bool {
	_, ok := self[v]

	return ok
}

func (self hashSet[K]) keys() []K {
	keys := []K{}

	for v := range self {
		keys = append(keys, v)
	}

	return keys
}

func (self hashSet[K]) intersection(other hashSet[K]) hashSet[K] {
	intersection := hashSet[K]{}

	for v := range self {
		if other.contains(v) {
			intersection[v] = struct{}{}
		}
	}

	return intersection
}

func part1(data []rucksak) uint {
	sum := uint(0)

	for _, item := range data {
		p1s := newHashSetFromSlice(item.p1.keys())
		p2s := newHashSetFromSlice(item.p2.keys())

		xsect := p1s.intersection(p2s)
		if len(xsect) > 0 {
			sum += itemToPriority(xsect.keys()[0])
		}
	}

	return sum
}

func part2(data []rucksak) uint {
	sum := uint(0)

	groups := [][]rucksak{}
	for i := 0; i < len(data); i += 3 {
		groups = append(groups, data[i:i+3])
	}

	for _, group := range groups {
		xsect := newHashSetFromSlice(append(group[0].p1.keys(), group[0].p2.keys()...))
		for _, item := range group[1:] {
			xsect = xsect.intersection(newHashSetFromSlice(append(item.p1.keys(), item.p2.keys()...)))
		}

		if len(xsect) > 0 {
			sum += itemToPriority(xsect.keys()[0])
		}
	}

	return sum
}

func itemToPriority(v byte) uint {
	if v >= 'a' && v <= 'z' {
		return uint(v - 'a' + 1)
	} else if v >= 'A' && v <= 'Z' {
		return uint(v - 'A' + 27)
	}

	return 0
}
