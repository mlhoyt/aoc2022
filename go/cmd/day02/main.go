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

	data1, err := parse(input, func(f1 string, f2 string) (partialRound, error) {
		return newRoundWithoutOutcome(f1, f2)
	})
	if err != nil {
		panic(err)
	}
	// fmt.Printf("%s\n", data1)

	result1 := uint(0)
	for _, round := range data1 {
		result1 += round.eval().points()
	}
	fmt.Println("part1: ", result1)

	data2, err := parse(input, func(f1 string, f2 string) (partialRound, error) {
		return newRoundWithoutYourPlay(f1, f2)
	})
	if err != nil {
		panic(err)
	}
	// fmt.Printf("%s\n", data2)

	result2 := uint(0)
	for _, round := range data2 {
		result2 += round.eval().points()
	}
	fmt.Println("part2: ", result2)
}

// partialRound is an abstraction over various round* types that can be "evaluated" into a complete
// round
type partialRound interface {
	eval() round
}

// shape defines the enumerations of Rock, Paper, and Scissors
type shape uint

// shape enumeration values
const (
	Rock shape = iota + 1
	Paper
	Scissors
)

func (self shape) points() uint {
	return uint(self)
}

// outcome defines the result enumerations
type outcome uint

// outcome enumeration values
const (
	Win  outcome = 6
	Lose         = 0
	Draw         = 3
)

func (self outcome) points() uint {
	return uint(self)
}

type round struct {
	oppPlay  shape
	yourPlay shape
	outcome  outcome
}

func (self round) points() uint {
	return self.yourPlay.points() + self.outcome.points()
}

func parse(input string, mapFn func(f1 string, f2 string) (partialRound, error)) ([]partialRound, error) {
	rounds := []partialRound{}

	for _, line := range strings.Split(input, "\n") {
		fields := strings.Split(line, " ")
		round, err := mapFn(fields[0], fields[1])
		if err != nil {
			return nil, err
		}

		rounds = append(rounds, round)
	}

	return rounds, nil
}

type roundWithoutOutcome struct {
	oppPlay  shape
	yourPlay shape
}

func newRoundWithoutOutcome(f1 string, f2 string) (roundWithoutOutcome, error) {
	var oppPlay shape
	switch f1 {
	case "A":
		oppPlay = Rock
	case "B":
		oppPlay = Paper
	case "C":
		oppPlay = Scissors
	default:
		return roundWithoutOutcome{}, fmt.Errorf("cannot parse string %q into shape", f1)
	}

	var yourPlay shape
	switch f2 {
	case "X":
		yourPlay = Rock
	case "Y":
		yourPlay = Paper
	case "Z":
		yourPlay = Scissors
	default:
		return roundWithoutOutcome{}, fmt.Errorf("cannot parse string %q into shape", f2)
	}

	return roundWithoutOutcome{
		oppPlay,
		yourPlay,
	}, nil
}

func (self roundWithoutOutcome) determineOutcome() outcome {
	var outcome outcome
	switch self.yourPlay {
	case Rock:
		switch self.oppPlay {
		case Rock:
			outcome = Draw
		case Paper:
			outcome = Lose
		case Scissors:
			outcome = Win
		}
	case Paper:
		switch self.oppPlay {
		case Rock:
			outcome = Win
		case Paper:
			outcome = Draw
		case Scissors:
			outcome = Lose
		}
	case Scissors:
		switch self.oppPlay {
		case Rock:
			outcome = Lose
		case Paper:
			outcome = Win
		case Scissors:
			outcome = Draw
		}
	}

	return outcome
}

func (self roundWithoutOutcome) eval() round {
	return round{
		oppPlay:  self.oppPlay,
		yourPlay: self.yourPlay,
		outcome:  self.determineOutcome(),
	}
}

type roundWithoutYourPlay struct {
	oppPlay shape
	outcome outcome
}

func newRoundWithoutYourPlay(f1 string, f2 string) (roundWithoutYourPlay, error) {
	var oppPlay shape
	switch f1 {
	case "A":
		oppPlay = Rock
	case "B":
		oppPlay = Paper
	case "C":
		oppPlay = Scissors
	default:
		return roundWithoutYourPlay{}, fmt.Errorf("cannot parse string %q into shape", f1)
	}

	var outcome outcome
	switch f2 {
	case "X":
		outcome = Lose
	case "Y":
		outcome = Draw
	case "Z":
		outcome = Win
	default:
		return roundWithoutYourPlay{}, fmt.Errorf("cannot parse string %q into shape", f2)
	}

	return roundWithoutYourPlay{
		oppPlay,
		outcome,
	}, nil
}

func (self roundWithoutYourPlay) determineYourPlay() shape {
	var yourPlay shape
	switch self.outcome {
	case Win:
		switch self.oppPlay {
		case Rock:
			yourPlay = Paper
		case Paper:
			yourPlay = Scissors
		case Scissors:
			yourPlay = Rock
		}
	case Draw:
		yourPlay = self.oppPlay
	case Lose:
		switch self.oppPlay {
		case Rock:
			yourPlay = Scissors
		case Paper:
			yourPlay = Rock
		case Scissors:
			yourPlay = Paper
		}
	}

	return yourPlay
}

func (self roundWithoutYourPlay) eval() round {
	return round{
		oppPlay:  self.oppPlay,
		yourPlay: self.determineYourPlay(),
		outcome:  self.outcome,
	}
}
