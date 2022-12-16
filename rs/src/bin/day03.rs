use aoc;
use std::collections::{HashMap, HashSet};

fn main() {
    // input is a String
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{}", input);

    // data is a [(RucksakPocket, RucksakPocket)]
    // ... which is a [(HashMap<Item, usize>, HashMap<Item, usize>)]
    // ... which is a [(HashMap<char, usize>, HashMap<char, usize>)]
    let data = parse_input(&input).expect("cannot parse input");
    // println!("{:?}", data);

    let result1: usize = data
        .iter()
        .map(|(p1, p2)| {
            let p1s = p1.keys().collect::<HashSet<_>>();
            let p2s = p2.keys().collect::<HashSet<_>>();

            p1s.intersection(&p2s)
                .copied()
                .last()
                .expect("empty intersection of rucksak pocket contents")
                .to_priority()
                .expect("item has no priority")
        })
        .sum();
    println!("part1: {}", result1);

    let result2: usize = data
        .chunks(3)
        .map(|g| {
            // TODO: Should check that g.len() == 3

            let g = g
                .iter()
                .map(|(p1, p2)| {
                    let p1s = p1.keys().collect::<HashSet<_>>();
                    let p2s = p2.keys().collect::<HashSet<_>>();
                    p1s.union(&p2s).copied().collect::<HashSet<_>>()
                })
                .collect::<Vec<_>>();

            g[1..]
                .iter()
                .fold(g[0].clone(), |acc, v| {
                    acc.intersection(v).copied().collect()
                })
                .iter()
                .last()
                .expect("empty intersection of rucksak contents")
                .to_priority()
                .expect("item has no priority")
        })
        .sum();
    println!("part2: {:?}", result2);
}

type Item = char;

type RucksakPocket = HashMap<Item, usize>;

fn parse_input(input: &str) -> Result<Vec<(RucksakPocket, RucksakPocket)>, String> {
    let str_to_rucksak_pocket = |s: &str| -> RucksakPocket {
        s.chars().fold(RucksakPocket::new(), |mut acc, v| {
            *acc.entry(v).or_insert(0) += 1;
            acc
        })
    };

    Ok(input
        .lines()
        .map(|l| {
            let (h1, h2) = l.trim().split_at(l.len() / 2);

            (str_to_rucksak_pocket(h1), str_to_rucksak_pocket(h2))
        })
        .collect())
}

trait Priority {
    fn to_priority(&self) -> Option<usize>;
}

impl Priority for Item {
    fn to_priority(&self) -> Option<usize> {
        match *self {
            'a'..='z' => Some(*self as usize - 'a' as usize + 1),
            'A'..='Z' => Some(*self as usize - 'A' as usize + 27),
            _ => None,
        }
    }
}
