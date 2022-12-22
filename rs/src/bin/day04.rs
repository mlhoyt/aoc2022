use aoc;

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{}", input);

    let data = parse(&input);
    // println!("{:?}", data);

    let result1 = part1(&data);
    println!("part1: {}", result1);

    let result2 = part2(&data);
    println!("part2: {}", result2);
}

fn parse(input: &str) -> Vec<(Assignment, Assignment)> {
    input
        .lines()
        .map(|s| {
            let vs: Vec<_> = s
                .split(",")
                .map(|s| {
                    let vs: Vec<usize> = s
                        .split("-")
                        .map(|s| s.parse())
                        .collect::<Result<_, _>>()
                        .expect("cannot parse input assignment");

                    Assignment {
                        min: vs[0],
                        max: vs[1],
                    }
                })
                .collect();

            (vs[0], vs[1])
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, Copy)]
struct Assignment {
    min: usize,
    max: usize,
}

impl Assignment {
    fn contains(&self, other: &Self) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.min >= other.min && self.min <= other.max)
            || (self.max >= other.min && self.max <= other.max)
    }
}

fn part1(data: &[(Assignment, Assignment)]) -> usize {
    data.iter()
        .map(|(a1, a2)| {
            if a1.contains(a2) || a2.contains(a1) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn part2(data: &[(Assignment, Assignment)]) -> usize {
    data.iter()
        .map(|(a1, a2)| {
            if a1.overlaps(a2) || a1.contains(a2) {
                1
            } else {
                0
            }
        })
        .sum()
}
