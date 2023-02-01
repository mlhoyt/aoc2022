use aoc;

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{}", input);

    let data = parse(&input).expect("cannot parse input");
    // println!("{:?}", data);

    let states = simulate(&data);

    let result1 = part1(&states);
    println!("part1: {}", result1);

    let result2 = part2(&states);
    println!("part2: {}", result2);
}

fn parse(input: &str) -> Result<Data, String> {
    input
        .lines()
        .map(|l| {
            let mut fields = l.split_whitespace();
            match fields.next() {
                Some("addx") => match fields.next() {
                    Some(v) => match v.parse::<isize>() {
                        Ok(n) => Ok(Instruction::Addx(n)),
                        Err(e) => Err(e.to_string()),
                    },
                    _ => Err("addx instruction requires one integer argument".into()),
                },
                Some("noop") => Ok(Instruction::Noop),
                _ => Err("unrecognized instruction".into()),
            }
        })
        .collect::<Result<_, _>>()
}

type Data = Vec<Instruction>;

#[derive(Debug)]
enum Instruction {
    Addx(isize),
    Noop,
}

fn simulate(data: &Data) -> Vec<isize> {
    // The resulting vector represents the value of the X register at the BEGINING of (and during)
    // the corresponding cycle.

    data.iter().fold(vec![1, 1], |mut acc, v| {
        let curr = acc.last().unwrap();
        let mut next = match v {
            Instruction::Addx(n) => {
                vec![*curr, *curr + n]
            }
            Instruction::Noop => {
                vec![*curr]
            }
        };
        acc.append(&mut next);
        acc
    })
}

fn part1(states: &[isize]) -> isize {
    states
        .iter()
        .enumerate()
        .skip(20)
        .filter(|(i, _)| (i - 20) % 40 == 0)
        .map(|(i, v)| i as isize * v)
        .sum()
}

fn part2(states: &[isize]) -> usize {
    states
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, v)| {
            let x = ((i - 1) % 40) as isize;
            if x >= v - 1 && x <= v + 1 {
                "#"
            } else {
                "."
            }
        })
        .collect::<Vec<_>>()
        .chunks(40)
        .for_each(|l| println!("{}", l.join("")));

    0
}
