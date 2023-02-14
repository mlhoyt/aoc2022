use aoc::grid2d::Grid2D;
use std::collections::{HashMap, VecDeque};

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{}", input);

    let data = parse(&input).expect("cannot parse input");
    // println!("{:?}", data);

    let result1 = part1(&data);
    println!("part1: {result1}");

    let result2 = part2(&data);
    println!("part2: {result2}");
}

fn parse(input: &str) -> Result<Data, String> {
    let mut start: Point<usize> = Point::default();
    let mut end: Point<usize> = Point::default();
    let grid_data: Vec<Vec<usize>> = input
        .lines()
        .enumerate()
        .map(|(rn, l)| {
            l.chars()
                .enumerate()
                .map(|(cn, c)| {
                    if c == 'S' {
                        start = (rn, cn);

                        0
                    } else if c == 'E' {
                        end = (rn, cn);

                        (b'z' - b'a') as usize
                    } else {
                        (c as u32 - 'a' as u32) as usize
                    }
                })
                .collect()
        })
        .collect();

    let grid = Grid2D::<_>::new(&grid_data);
    if grid.is_err() {
        return Err("unable to create Grid2D from grid data".into());
    }
    let grid = grid.unwrap();

    Ok(Data { grid, start, end })
}

#[derive(Debug)]
struct Data {
    grid: Grid2D<usize>,
    start: (usize, usize),
    end: (usize, usize),
}

type Point<T> = (T, T);

trait Shift {
    fn shift(self, offset: Point<isize>) -> Option<Self>
    where
        Self: Sized;
}

impl Shift for Point<usize> {
    fn shift(self, offset: Point<isize>) -> Option<Self> {
        let p0 = self.0 as isize + offset.0;
        let p1 = self.1 as isize + offset.1;

        if p0 < 0 || p1 < 0 {
            None
        } else {
            Some((p0 as usize, p1 as usize))
        }
    }
}

fn part1(data: &Data) -> usize {
    shortest_path(data, data.start).unwrap_or(0)
}

fn part2(data: &Data) -> usize {
    data.grid
        .iter()
        // find the valid starting points
        .filter(|p| p.value == 0)
        // transform each starting point to its minimum path length
        .filter_map(|p| shortest_path(data, (p.y, p.x)))
        // find the minimum path length from any of the starting points
        .min()
        .unwrap_or(0)
}

fn shortest_path(data: &Data, startpoint: Point<usize>) -> Option<usize> {
    let mut visited: HashMap<Point<usize>, usize> = HashMap::new();
    visited.insert(startpoint, 0);
    let mut endpoints: VecDeque<Point<usize>> = VecDeque::new();
    endpoints.push_back(startpoint);

    while !endpoints.is_empty() {
        let endpoint = endpoints.pop_front().unwrap();
        let endpoint_elevation = data.grid.get_yx(endpoint.0, endpoint.1).unwrap();
        let endpoint_steps = *visited.get(&endpoint).unwrap();

        let moves: Vec<_> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
            .into_iter()
            // adjacent cells above lower grid boundary
            .filter_map(|o| endpoint.shift(o))
            // adjacent cells below upper grid boundary
            .filter(|next| data.grid.get_yx(next.0, next.1).is_some())
            // adjacent cells not visited and not already an endpoint
            .filter(|next| !visited.contains_key(next) && !endpoints.contains(next))
            // adjacent cells with acceptable elevation difference
            .filter(|next| {
                let next_elevation = data.grid.get_yx(next.0, next.1).unwrap();

                next_elevation as isize - endpoint_elevation as isize <= 1
            })
            .collect();

        for next in moves {
            visited.insert(next, endpoint_steps + 1);

            if next == data.end {
                endpoints.clear();
                break;
            } else {
                endpoints.push_back(next);
            }
        }
    }

    visited.get(&data.end).copied()
}
