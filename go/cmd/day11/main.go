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

	// result2 := part2(data)
	// fmt.Printf("part2: %d\n", result2)
}

func parse(input string) (data, error) {
	data := data{}
	for _, s := range strings.Split(input, "\n\n") {
		monkey, err := newMonkey(s)
		if err != nil {
			return nil, err
		}

		data = append(data, monkey)
	}

	return data, nil
}

type data = []monkey

type monkey struct {
	items            []uint64
	inspectOperation inspectOperation
	inspectValue     inspectValue
	decisionFactor   uint64
	testOnTrue       uint
	testOnFalse      uint
}

// inspectOperation defines the operation of inspect amplification
type inspectOperation uint

// inspectOperation enumeration values
const (
	inspectOperationAdd inspectOperation = iota + 1
	inspectOperationMultiply
)

func (self inspectOperation) String() string {
	switch self {
	case inspectOperationAdd:
		return "+"
	case inspectOperationMultiply:
		return "*"
	default:
		return "undefined"
	}
}

type inspectValue struct {
	value uint64
	isOld bool
}

func newInspectValueLiteral(v uint64) inspectValue {
	return inspectValue{value: v, isOld: false}
}

func newInspectValueOld() inspectValue {
	return inspectValue{value: 0, isOld: true}
}

func newMonkey(input string) (monkey, error) {
	monkey := monkey{}
	hasItems := false
	hasInspectOperation := false
	hasInspectValue := false
	hasDecisionFactor := false
	hasTestOnTrue := false
	hasTestOnFalse := false

	for _, l := range strings.Split(input, "\n") {
		l = strings.TrimSpace(l)

		if strings.HasPrefix(l, "Monkey ") {
			// do nothing
		} else if strings.HasPrefix(l, "Starting items: ") {
			l = strings.TrimPrefix(l, "Starting items: ")

			for _, v := range strings.Split(l, ", ") {
				n, err := strconv.Atoi(v)
				if err != nil {
					return monkey, err
				}

				monkey.items = append(monkey.items, uint64(n))
				hasItems = true
			}
		} else if strings.HasPrefix(l, "Operation: new = old ") {
			l = strings.TrimPrefix(l, "Operation: new = old ")

			if strings.HasPrefix(l, "+") {
				monkey.inspectOperation = inspectOperationAdd
				hasInspectOperation = true
				l = strings.TrimSpace(strings.TrimPrefix(l, "+"))
			} else if strings.HasPrefix(l, "*") {
				monkey.inspectOperation = inspectOperationMultiply
				hasInspectOperation = true
				l = strings.TrimSpace(strings.TrimPrefix(l, "*"))
			} else {
				return monkey, fmt.Errorf("unexpected inspect operation: %q", l)
			}

			if strings.HasPrefix(l, "old") {
				monkey.inspectValue = newInspectValueOld()
				hasInspectValue = true
			} else {
				n, err := strconv.Atoi(l)
				if err != nil {
					return monkey, err
				}

				monkey.inspectValue = newInspectValueLiteral(uint64(n))
				hasInspectValue = true
			}
		} else if strings.HasPrefix(l, "Test: divisible by ") {
			l = strings.TrimPrefix(l, "Test: divisible by ")

			n, err := strconv.Atoi(l)
			if err != nil {
				return monkey, err
			}

			monkey.decisionFactor = uint64(n)
			hasDecisionFactor = true
		} else if strings.HasPrefix(l, "If true: throw to monkey ") {
			l = strings.TrimPrefix(l, "If true: throw to monkey ")

			n, err := strconv.Atoi(l)
			if err != nil {
				return monkey, err
			}

			monkey.testOnTrue = uint(n)
			hasTestOnTrue = true
		} else if strings.HasPrefix(l, "If false: throw to monkey ") {
			l = strings.TrimPrefix(l, "If false: throw to monkey ")

			n, err := strconv.Atoi(l)
			if err != nil {
				return monkey, err
			}

			monkey.testOnFalse = uint(n)
			hasTestOnFalse = true
		}
	}

	if !hasItems || !hasInspectOperation || !hasInspectValue || !hasDecisionFactor || !hasTestOnTrue || !hasTestOnFalse {
		return monkey, fmt.Errorf("parsed input but some fields missing")
	}

	return monkey, nil
}

func part1(data data) uint64 {
	items := [][]uint64{}
	inspectCounts := []uint{}
	for _, v := range data {
		items = append(items, v.items)
		inspectCounts = append(inspectCounts, 0)
	}

	for i := 0; i < 20; i++ {
		items, inspectCounts = simulateRound(data, items, inspectCounts)
	}

	sort.Slice(inspectCounts, func(i, j int) bool {
		return inspectCounts[i] > inspectCounts[j] // DESC
	})

	product := uint64(1)
	for _, v := range inspectCounts[:2] {
		product *= uint64(v)
	}

	return product
}

func simulateRound(data data, items [][]uint64, inspectCounts []uint) ([][]uint64, []uint) {
	for i := 0; i < len(items); i++ {
		for len(items[i]) > 0 {
			v := items[i][0]
			items[i] = items[i][1:]
			// fmt.Printf("Monkey %d inspects item with worry level %d\n", i, v)

			// Inspect
			inspectCounts[i] += 1

			iv := v
			if !data[i].inspectValue.isOld {
				iv = data[i].inspectValue.value
			}

			switch data[i].inspectOperation {
			case inspectOperationAdd:
				v += iv
				// fmt.Printf("- the inspection increases the worry level by %s %d to %d\n", data[i].inspectOperation, iv, v)
			case inspectOperationMultiply:
				v *= iv
				// fmt.Printf("- the inspection increases the worry level by %s %d to %d\n", data[i].inspectOperation, iv, v)
			}

			// Relief
			v /= 3
			// fmt.Printf("-- the inspection finishes decreasing the worry level to %d\n", v)

			// Decide
			ip := data[i].testOnTrue
			if v%data[i].decisionFactor == 0 {
				ip = data[i].testOnTrue
				// fmt.Printf("--- the worry level IS divisible by %d\n", data[i].decisionFactor)
			} else {
				ip = data[i].testOnFalse
				// fmt.Printf("--- the worry level is NOT divisible by %d\n", data[i].decisionFactor)
			}

			// Throw
			items[ip] = append(items[ip], v)
			// fmt.Printf("---- the item is thrown to monkey %d\n", ip)
		}
	}

	return items, inspectCounts
}
