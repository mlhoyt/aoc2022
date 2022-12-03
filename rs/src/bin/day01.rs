use aoc;

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");

    let data: Vec<Vec<u32>> = input
        .split("\n\n")
        .map(|s| {
            s.trim()
                .split_whitespace()
                .map(|v| v.parse())
                .collect::<Result<Vec<_>, _>>()
                .expect("cannot parse input")
        })
        .collect();

    let result1: u32 = data.clone().iter().map(|v| v.iter().sum()).max().unwrap();
    println!("part1: {:?}", result1);

    let mut result2 = data
        .clone()
        .iter()
        .map(|v| v.iter().sum::<u32>())
        .collect::<Vec<_>>();
    result2.sort();
    result2.reverse();
    let result2: u32 = result2.iter().take(3).sum();
    println!("part2: {:?}", result2);
}
