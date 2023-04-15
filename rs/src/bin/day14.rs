fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{}", input);

    let data = parse(&input).expect("cannot parse input");
    // println!("{:?}", data);

    let source_point = Point2D { x: 500, y: 0 };
    let bottom = data.iter().map(|(p, _)| p.y).max().unwrap();

    let term1_fn = |p: &Point2D| p.y >= bottom;
    let term2_fn = |p: &Point2D| p.y >= (bottom + 2);
    let noop_fn = |_p: &Point2D| false;

    let result1 = simulate(&data, &source_point, &noop_fn, &term1_fn);
    println!("part1: {result1}");

    let result2 = simulate(&data, &source_point, &term2_fn, &noop_fn);
    println!("part2: {result2}");
}

fn parse(input: &str) -> Result<Data, String> {
    let mut data = Data::new();

    input.lines().for_each(|l| {
        // Extract each "x,y" point and convert to a Point2D{x,y}
        let points: Vec<_> = l
            .split(" -> ")
            .map(|p| {
                let vs = p
                    .split(",")
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                Point2D { x: vs[0], y: vs[1] }
            })
            .collect();

        // Convert each pair of adjacent points (a line) into each individual point and store in
        // the HashMap.
        points.windows(2).for_each(|ps| {
            if ps[0].x == ps[1].x {
                let ys = if ps[0].y < ps[1].y {
                    ps[0].y..=ps[1].y
                } else {
                    ps[1].y..=ps[0].y
                };
                for y in ys {
                    data.insert(Point2D { x: ps[0].x, y }, PointType::Rock);
                }
            } else {
                let xs = if ps[0].x < ps[1].x {
                    ps[0].x..=ps[1].x
                } else {
                    ps[1].x..=ps[0].x
                };
                for x in xs {
                    data.insert(Point2D { x, y: ps[0].y }, PointType::Rock);
                }
            }
        });
    });

    Ok(data)
}

type Data = std::collections::HashMap<Point2D, PointType>;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point2D {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
enum PointType {
    Rock,
    Sand,
}

fn simulate<FF: Fn(&Point2D) -> bool, FT: Fn(&Point2D) -> bool>(
    data: &Data,
    start_point: &Point2D,
    floor_fn: &FF,
    term_fn: &FT,
) -> usize {
    let mut data = data.clone();

    loop {
        match step(&data, &start_point, &floor_fn, &term_fn) {
            Some(valid_point) => {
                data.insert(valid_point, PointType::Sand);

                // part2 termination
                if valid_point == *start_point {
                    break;
                }
            }
            // part1 termination
            _ => {
                break;
            }
        }

        // show_state(&data);
    }

    data.iter()
        .filter(|(_, t)| match *t {
            PointType::Sand => true,
            _ => false,
        })
        .count()
}

// A unit of sand always falls down one step if possible.
// If the tile immediately below is blocked then the sand attempts to move diagonally one step down
// and to the left.
// If the tile down and to the left is blocked then the sand attempts to move diagonally one step down and to the right.
// If all three destinations are blocked then the sand comes to rest.
// If a unit of sand can move to a destination below the bottom then stop.
// Count the number of units of sand that came to rest.
fn step<FF: Fn(&Point2D) -> bool, FT: Fn(&Point2D) -> bool>(
    data: &Data,
    start_point: &Point2D,
    floor_fn: &FF,
    term_fn: &FT,
) -> Option<Point2D> {
    let mut curr_point = start_point.clone();

    let mut cont = true;
    while cont {
        let moves: Vec<_> = vec![
            Point2D {
                x: curr_point.x,
                y: curr_point.y + 1,
            },
            Point2D {
                x: curr_point.x - 1,
                y: curr_point.y + 1,
            },
            Point2D {
                x: curr_point.x + 1,
                y: curr_point.y + 1,
            },
        ]
        .into_iter()
        .filter_map(|p| {
            if data.get(&p).is_none() && !floor_fn(&p) {
                Some(p)
            } else {
                None
            }
        })
        .collect();

        match moves.is_empty() {
            // at rest
            true => {
                return Some(curr_point);
            }
            // moved
            false => {
                curr_point = moves[0];
            }
        }

        if term_fn(&curr_point) {
            cont = false;
        }
    }

    None
}

fn show_state(data: &Data) {
    let x_min = data.iter().map(|(p, _)| p.x).min().unwrap();
    let x_max = data.iter().map(|(p, _)| p.x).max().unwrap();
    let y_max = data.iter().map(|(p, _)| p.y).max().unwrap();

    for y in 0..=(y_max + 1) {
        for x in (x_min - 1)..=(x_max + 1) {
            match data.get(&Point2D { x, y }) {
                None => {
                    print!(".");
                }
                Some(&v) => match v {
                    PointType::Rock => {
                        print!("#");
                    }
                    PointType::Sand => {
                        print!("o");
                    }
                },
            }
        }
        println!("");
    }
}
