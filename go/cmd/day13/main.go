package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
	"unicode"

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
	fmt.Printf("%#v\n", data)

	result1 := part1(data)
	fmt.Printf("part1: %v\n", result1)

	result2 := part2(data)
	fmt.Printf("part2: %v\n", result2)
}

func parse(input string) (data, error) {
	packets := []node{}

	for _, line := range strings.Split(input, "\n") {
		if line == "" {
			continue
		}

		packetData, err := newNodeFromString(line)
		if err != nil {
			return data{}, err
		}

		packets = append(packets, packetData)
	}

	data := data{pairs: [][]node{}}
	for i := 0; i < len(packets); i += 2 {
		data.pairs = append(data.pairs, []node{packets[i], packets[i+1]})
	}

	return data, nil
}

type data struct {
	pairs [][]node
}

type node interface {
	isNode()
	cmp(node) int
}

type valueNode struct {
	value uint
}

func (self valueNode) isNode() {}

func (self valueNode) cmp(other node) int {
	switch otherp := other.(type) {
	case valueNode:
		if self.value < otherp.value {
			return -1
		} else if self.value == otherp.value {
			return 0
		} else {
			return 1
		}
	case listNode:
		return listNode{value: []node{self}}.cmp(other)
	default:
		return 0 // unreachable
	}
}

type listNode struct {
	value []node
}

func (self listNode) isNode() {}

func (self listNode) cmp(other node) int {
	switch otherp := other.(type) {
	case valueNode:
		return self.cmp(listNode{value: []node{other}})
	case listNode:
		result := 0
		for i := 0; i < len(self.value); i++ {
			if i >= len(otherp.value) {
				result = 1
				break
			}

			result = self.value[i].cmp(otherp.value[i])
			if result != 0 {
				break
			}
		}

		if result == 0 && len(self.value) < len(otherp.value) {
			result = -1
		}

		return result
	default:
		return 0 // unreachable
	}
}

func newNodeFromString(s string) (node, error) {
	if strings.HasPrefix(s, "[") {
		if !strings.HasSuffix(s, "]") {
			return nil, fmt.Errorf("packet-data list string must start with [ and end with ]")
		}

		s = strings.TrimSuffix(strings.TrimPrefix(s, "["), "]")

		data := []node{}
		for s != "" {
			if strings.HasPrefix(s, "[") {
				n, ok := findToMatchingBracket(s)
				if !ok {
					return nil, fmt.Errorf("packet data list string must start with [ and end with ]")
				}

				packetData, rest := s[:n+1], s[n+1:]

				v, err := newNodeFromString(packetData)
				if err != nil {
					return nil, err
				}

				data = append(data, v)

				s = rest
			} else if unicode.IsDigit([]rune(s)[0]) {
				n := findToEndOfNumber(s)

				packetData, rest := s[:n+1], s[n+1:]

				v, err := newNodeFromString(packetData)
				if err != nil {
					return nil, err
				}

				data = append(data, v)

				s = rest
			} else {
				return nil, fmt.Errorf("unexpected packet-data string")
			}

			s = strings.TrimPrefix(s, ",")
		}

		return listNode{value: data}, nil
	} else if unicode.IsDigit([]rune(s)[0]) {
		v, err := strconv.Atoi(s)
		if err != nil {
			return nil, err
		}

		return valueNode{value: uint(v)}, nil
	} else {
		return nil, fmt.Errorf("unexpected packet-data string")
	}
}

func findToMatchingBracket(s string) (uint, bool) {
	c, n, x := 0, 0, false
	for _, sn := range []rune(s) {
		// if not done then update the next-state
		if !x {
			// update the depth of brackets
			cp := c
			if sn == '[' {
				cp = c + 1
			} else if sn == ']' {
				cp = c - 1
			}

			// update the position of the end
			np := n
			if cp != 0 {
				np = n + 1
			}

			// update the done condition
			xp := false
			if cp == 0 {
				xp = true
			}

			// state = next-state
			c = cp
			n = np
			x = xp
		}
	}

	return uint(n), x
}

func findToEndOfNumber(s string) uint {
	n, cont := -1, true
	for _, sn := range []rune(s) {
		// if not done and next is digiti then update the next-state
		if cont && unicode.IsDigit(sn) {
			n = n + 1
		} else {
			cont = false
		}
	}

	return uint(n)
}

func part1(data data) uint {
	result := uint(0)

	for i, pair := range data.pairs {
		if pair[0].cmp(pair[1]) < 0 {
			result += uint(i + 1)
		}
	}

	return result
}

func part2(data data) uint {
	allPackets := []node{}
	for _, pair := range data.pairs {
		allPackets = append(allPackets, pair[0], pair[1])
	}

	div1, _ := newNodeFromString("[[2]]")
	div2, _ := newNodeFromString("[[6]]")
	allPackets = append(allPackets, div1, div2)

	sort.Slice(allPackets, func(i int, j int) bool {
		cmp := allPackets[i].cmp(allPackets[j])
		if cmp > 0 {
			return false
		}

		return true
	})

	result := uint(1)
	for i, packet := range allPackets {
		if packet.cmp(div1) == 0 || packet.cmp(div2) == 0 {
			result *= uint(i + 1)
		}
	}

	return result
}
