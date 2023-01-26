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
            let vs: Vec<_> = l.split_whitespace().collect();

            if vs.len() != 2 {
                return Err("too few values; expected direction and distance".into());
            }

            let dir = match vs[0] {
                "U" => Some((0 as isize, 1 as isize)),
                "R" => Some((1 as isize, 0 as isize)),
                "D" => Some((0 as isize, -1 as isize)),
                "L" => Some((-1 as isize, 0 as isize)),
                _ => None,
            };
            if dir.is_none() {
                return Err("cannot parse direction".into());
            }
            let dir = dir.unwrap();

            let dist: Result<usize, _> = vs[1].parse();
            if dist.is_err() {
                return Err("cannot parse distance".into());
            }
            let dist = dist.unwrap();

            Ok(std::iter::repeat(dir).take(dist).collect::<Vec<_>>())
        })
        .flat_map(|result| match result {
            Ok(vec) => vec.into_iter().map(|v| Ok(v)).collect(),
            Err(e) => vec![Err(e)],
        })
        .collect::<Result<_, _>>()
}

type Data = Vec<Point>;

type Point = (isize, isize);

// trait Adder {
//     fn add(&self, other: &Self) -> Self;
// }

// impl Adder for Point {
//     fn add(&self, other: &Self) -> Self {
//         (self.0 + other.0, self.1 + other.1)
//     }
// }

// trait Follower {
//     fn follow(&self, other: &Self) -> Self;
// }

// impl Follower for Point {
//     fn follow(&self, other: &Self) -> Self {
//         let dx = other.0 - self.0;
//         let dy = other.1 - self.1;

//         let xp = if dx != 0 {
//             self.0 + (dx / dx.abs())
//         } else {
//             self.0
//         };
//         let yp = if dy != 0 {
//             self.1 + (dy / dy.abs())
//         } else {
//             self.1
//         };

//         if dx.abs() > 1 || dy.abs() > 1 {
//             (xp, yp)
//         } else {
//             self.clone()
//         }
//     }
// }

fn part1(data: &Data) -> usize {
    let mut head = Point::default();
    let mut tail = Point::default();

    let mut ps: std::collections::HashSet<Point> = std::collections::HashSet::new();

    data.iter().for_each(|(dx, dy)| {
        head = (head.0 + dx, head.1 + dy);
        tail = update_tail(&head, &tail);
        // head = head.add(&(*dx, *dy));
        // tail = tail.follow(&head);

        ps.insert(tail.clone());
    });

    ps.len()
}

fn update_tail(head: &Point, tail: &Point) -> Point {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;

    let xp = if dx != 0 { tail.0 + (dx / dx.abs()) } else { 0 };
    let yp = if dy != 0 { tail.1 + (dy / dy.abs()) } else { 0 };

    if dx == 0 && dy.abs() > 1 {
        (tail.0, yp)
    } else if dy == 0 && dx.abs() > 1 {
        (xp, tail.1)
    } else if dx != 0 && dy != 0 && (dx.abs() > 1 || dy.abs() > 1) {
        (xp, yp)
    } else {
        (tail.0, tail.1)
    }
}

fn part2(data: &Data) -> usize {
    let mut chain = vec![Point::default(); 10];

    let mut ps: std::collections::HashSet<Point> = std::collections::HashSet::new();

    data.iter().for_each(|(dx, dy)| {
        let mut head = chain[0];
        head = (head.0 + dx, head.1 + dy);
        chain[0] = head;

        for i in 1..chain.len() {
            let prev = chain[i - 1];
            let mut curr = chain[i];
            curr = update_tail(&prev, &curr);
            chain[i] = curr;
        }

        ps.insert(chain.last().unwrap().clone());
    });

    ps.len()
}
