use aoc;

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{}", input);

    let data = parse_input(&input, PartialRound::new_without_outcome);
    // println!("{:?}", data);

    let result1: usize = data
        .iter()
        .map(|pr| Round::from(pr))
        .map(|r| r.points())
        .sum();
    println!("part1: {}", result1);

    let data = parse_input(&input, PartialRound::new_without_your_play);
    // println!("{:?}", data);

    let result2: usize = data
        .iter()
        .map(|pr| Round::from(pr))
        .map(|r| r.points())
        .sum();
    println!("part2: {}", result2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Rock, Self::Rock) => std::cmp::Ordering::Equal,
            (Self::Rock, Self::Paper) => std::cmp::Ordering::Less,
            (Self::Rock, Self::Scissors) => std::cmp::Ordering::Greater,
            (Self::Paper, Self::Rock) => std::cmp::Ordering::Greater,
            (Self::Paper, Self::Paper) => std::cmp::Ordering::Equal,
            (Self::Paper, Self::Scissors) => std::cmp::Ordering::Less,
            (Self::Scissors, Self::Rock) => std::cmp::Ordering::Less,
            (Self::Scissors, Self::Paper) => std::cmp::Ordering::Greater,
            (Self::Scissors, Self::Scissors) => std::cmp::Ordering::Equal,
        }
    }
}

impl Shape {
    fn points(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum PartialRound {
    WithoutOutcome {
        opp_play: Shape,
        your_play: Shape,
    },
    WithoutYourPlay {
        opp_play: Shape,
        outcome: std::cmp::Ordering,
    },
}

impl PartialRound {
    fn new_without_outcome(opp_play: &str, your_play: &str) -> PartialRound {
        let opp_play = match opp_play {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            _ => Err("cannot parse input token"),
        }
        .expect("cannot parse input");

        let your_play = match your_play {
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            _ => Err("cannot parse input token"),
        }
        .expect("cannot parse input");

        Self::WithoutOutcome {
            opp_play,
            your_play,
        }
    }

    fn new_without_your_play(opp_play: &str, outcome: &str) -> PartialRound {
        let opp_play = match opp_play {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            _ => Err("cannot parse input token"),
        }
        .expect("cannot parse input");

        let outcome = match outcome {
            "X" => Ok(std::cmp::Ordering::Less),
            "Y" => Ok(std::cmp::Ordering::Equal),
            "Z" => Ok(std::cmp::Ordering::Greater),
            _ => Err("cannot parse input token"),
        }
        .expect("cannot parse input");

        Self::WithoutYourPlay { opp_play, outcome }
    }
}

fn parse_input(input: &str, map_fn: fn(f1: &str, f2: &str) -> PartialRound) -> Vec<PartialRound> {
    input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        // TODO: Should check each value has two elements
        .map(|vs| map_fn(vs[0], vs[1]))
        .collect()
}

#[derive(Debug)]
struct Round {
    opp_play: Shape,
    your_play: Shape,
    outcome: std::cmp::Ordering,
}

impl Round {
    fn points(&self) -> usize {
        self.your_play.points()
            + match self.outcome {
                std::cmp::Ordering::Less => 0,
                std::cmp::Ordering::Equal => 3,
                std::cmp::Ordering::Greater => 6,
            }
    }
}

impl From<&PartialRound> for Round {
    fn from(item: &PartialRound) -> Self {
        match item {
            PartialRound::WithoutOutcome {
                opp_play,
                your_play,
            } => Self {
                opp_play: opp_play.clone(),
                your_play: your_play.clone(),
                outcome: your_play.cmp(opp_play),
            },
            PartialRound::WithoutYourPlay { opp_play, outcome } => {
                let your_play = match (outcome, opp_play) {
                    (std::cmp::Ordering::Greater, Shape::Rock) => Shape::Paper,
                    (std::cmp::Ordering::Equal, Shape::Rock) => Shape::Rock,
                    (std::cmp::Ordering::Less, Shape::Rock) => Shape::Scissors,
                    (std::cmp::Ordering::Greater, Shape::Paper) => Shape::Scissors,
                    (std::cmp::Ordering::Equal, Shape::Paper) => Shape::Paper,
                    (std::cmp::Ordering::Less, Shape::Paper) => Shape::Rock,
                    (std::cmp::Ordering::Greater, Shape::Scissors) => Shape::Rock,
                    (std::cmp::Ordering::Equal, Shape::Scissors) => Shape::Scissors,
                    (std::cmp::Ordering::Less, Shape::Scissors) => Shape::Paper,
                };

                Self {
                    opp_play: opp_play.clone(),
                    your_play,
                    outcome: outcome.clone(),
                }
            }
        }
    }
}
