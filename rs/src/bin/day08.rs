use aoc;

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{}", input);

    let data = parse(&input).expect("cannot parse input");
    // println!("{:?}", data);

    let result1 = part1(&data);
    println!("part1: {}", result1);

    let result2 = part2(&data);
    println!("part2: {}", result2);
}

fn parse(input: &str) -> Result<Data, String> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or("cannot parse char to digit".to_string())
                })
                .collect()
        })
        .collect()
}

type Data = Vec<Vec<u32>>;

fn part1(data: &Data) -> usize {
    let mut can_be_seen = std::collections::HashSet::<(usize, usize)>::new();

    for r in 0..data.len() {
        for c in 0..data[r].len() {
            if r == 0 || r == data.len() - 1 || c == 0 || c == data[r].len() - 1 {
                can_be_seen.insert((r, c));
            } else {
                // from north
                if (0..r).rev().map(|rp| data[r][c] > data[rp][c]).all(|v| v) {
                    can_be_seen.insert((r, c));
                }

                // from east
                if (c + 1..data[r].len())
                    .map(|cp| data[r][c] > data[r][cp])
                    .all(|v| v)
                {
                    can_be_seen.insert((r, c));
                }

                // from south
                if (r + 1..data.len())
                    .map(|rp| data[r][c] > data[rp][c])
                    .all(|v| v)
                {
                    can_be_seen.insert((r, c));
                }

                // from west
                if (0..c).rev().map(|cp| data[r][c] > data[r][cp]).all(|v| v) {
                    can_be_seen.insert((r, c));
                }
            }
        }
    }

    can_be_seen.len()
}

fn part2(data: &Data) -> usize {
    let mut scenic_score = std::collections::HashMap::<(usize, usize), usize>::new();

    for r in 0..data.len() {
        for c in 0..data[r].len() {
            // looking north
            let (ssn, _) = (0..r)
                .rev()
                .fold((0, false), |(trees_visible, is_blocked), rp| {
                    if !is_blocked {
                        (trees_visible + 1, data[rp][c] >= data[r][c])
                    } else {
                        (trees_visible, is_blocked)
                    }
                });

            // looking east
            let (sse, _) =
                (c + 1..data[r].len()).fold((0, false), |(trees_visible, is_blocked), cp| {
                    if !is_blocked {
                        (trees_visible + 1, data[r][cp] >= data[r][c])
                    } else {
                        (trees_visible, is_blocked)
                    }
                });

            // looking south
            let (sss, _) =
                (r + 1..data.len()).fold((0, false), |(trees_visible, is_blocked), rp| {
                    if !is_blocked {
                        (trees_visible + 1, data[rp][c] >= data[r][c])
                    } else {
                        (trees_visible, is_blocked)
                    }
                });

            // looking west
            let (ssw, _) = (0..c)
                .rev()
                .fold((0, false), |(trees_visible, is_blocked), cp| {
                    if !is_blocked {
                        (trees_visible + 1, data[r][cp] >= data[r][c])
                    } else {
                        (trees_visible, is_blocked)
                    }
                });

            scenic_score.insert((r, c), (ssn * sse * sss * ssw) as usize);
        }
    }

    *scenic_score.iter().map(|(_, v)| v).max().unwrap()
}

// If trying to convert to functional style then creating a cartesian product of the row and
// column indexes would be useful.
// Alternatively, the itertools create has a cartesian product function.
//
// let rn = data.len();
// let cn = data[0].len();
// let rcs = (0..rn)
//     .flat_map(|r| std::iter::repeat(r).take(rn).zip(0..cn).collect::<Vec<_>>())
//     .collect::<Vec<_>>();
// // => [(0, 0), (0, 1), ..., (1, 0), (1, 1), ...]
