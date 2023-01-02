use aoc;

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{}", input);

    let result1 = part1(&input);
    println!("part1: {}", result1);

    let result2 = part2(&input);
    println!("part2: {}", result2);
}

fn part1(data: &str) -> usize {
    let sops: Vec<_> = data
        .as_bytes()
        .windows(4)
        .enumerate()
        .filter_map(|(i, w)| {
            let v1 = w.get(0).unwrap();
            let v2 = w.get(1).unwrap();
            let v3 = w.get(2).unwrap();
            let v4 = w.get(3).unwrap();
            if *v1 != *v2 && *v1 != *v3 && *v1 != *v4 && *v2 != *v3 && *v2 != *v4 && *v3 != *v4 {
                Some(i + 4)
            } else {
                None
            }
        })
        .collect();

    sops.first().unwrap().clone()
}

fn part2(data: &str) -> usize {
    let soms: Vec<_> = data
        .as_bytes()
        .windows(14)
        .enumerate()
        .filter_map(|(i, w)| {
            let set: std::collections::HashSet<&u8> = w.iter().collect();

            if set.len() == 14 {
                Some(i + 14)
            } else {
                None
            }
        })
        .collect();

    soms.first().unwrap().clone()
}
