fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{}", input);

    let data = parse(&input).expect("cannot parse input");
    println!("{:?}", data);

    let (min, max) = get_data_bounds(&data);
    println!("min={min:?} max={max:?}");

    let y = 10;
    // let y = 2_000_000;
    let result1 = (min.x..=max.x)
        .into_iter()
        .map(|x| point_to_status(&data, Point::new(x, y)))
        .filter(|v| *v == PointStatus::Covered)
        .count();
    println!("part1={result1}");

    // The following solution works for the test data but not the full data.
    // Reading through the fasterthanli.me solution: intead of processing every point it processes
    // every row computing covered ranges, coalescing the ranges, truncating to the predefined min
    // and max, and identifying the points between the ranges.  It is somewhat intuitive how that
    // would scale while computing every point would not.

    let xs = 0..=20;
    let ys = 0..=20;
    // let xs = 0..=4_000_000;
    // let ys = 0..=4_000_000;
    let result2 = 
        // Cartesian product iterator of xs and ys
        ys.flat_map(|y| xs.clone().map(move |x| Point::new(x, y)))
        .map(|p| (p.clone(), point_to_status(&data, p)))
        .filter(|(_, s)| *s == PointStatus::Uncovered)
        .collect::<Vec<_>>();
    println!("part2={result2:?}");
}

fn parse(input: &str) -> Result<Data, String> {
    Ok(input
        .lines()
        .map(|l| match parser::parse_sensor_line(l) {
            Err(e) => Err(format!("{e}")),
            Ok((sx, sy, bx, by)) => Ok((Point::new(sx, sy), Point::new(bx, by))),
        })
        .collect::<Result<Vec<(Point, Point)>, _>>()?
        .into_iter()
        .collect::<Data>())
}

type Data = std::collections::HashMap<Point, Point>;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

mod parser {
    use nom::{
        bytes::complete::tag, character::complete::i64, character::complete::space1, IResult,
    };

    pub fn parse_sensor_line(input: &str) -> Result<(i64, i64, i64, i64), String> {
        let result = parse_sensor_stmt(input);
        match result {
            Err(_) => Err("failed to parse input".into()),
            Ok((rest, vs)) => {
                if !rest.is_empty() {
                    Err(format!("did not consume all input: {rest}").into())
                } else {
                    Ok(vs)
                }
            }
        }
    }

    // Sample: Sensor at x=2, y=18: closest beacon is at x=-2, y=15

    fn parse_sensor_stmt(input: &str) -> IResult<&str, (i64, i64, i64, i64)> {
        let (input, _) = tag("Sensor")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("at")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("x=")(input)?;
        let (input, sensor_x) = i64(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("y=")(input)?;
        let (input, sensor_y) = i64(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("closest")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("beacon")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("is")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("at")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("x=")(input)?;
        let (input, beacon_x) = i64(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("y=")(input)?;
        let (input, beacon_y) = i64(input)?;

        Ok((input, (sensor_x, sensor_y, beacon_x, beacon_y)))
    }
}

fn get_data_bounds(data: &Data) -> (Point, Point) {
    data.iter()
        // Compute the bounds of each sensor-beacon pair
        .map(|(s, b)| {
            let r = s.distance(b);

            (Point::new(s.x - r, s.y - r), Point::new(s.x + r, s.y + r))
        })
        // Deterine the overall bounds based on the sensor-beacon bounds
        .fold(
            (
                Point::new(i64::MAX, i64::MAX),
                Point::new(i64::MIN, i64::MIN),
            ),
            |(acc_min, acc_max), (sb_min, sb_max)| {
                (
                    Point::new(acc_min.x.min(sb_min.x), acc_min.y.min(sb_min.y)),
                    Point::new(acc_max.x.max(sb_max.x), acc_max.y.max(sb_max.y)),
                )
            },
        )
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
enum PointStatus {
    Uncovered,
    Covered,
    Beacon,
    Sensor,
}

fn point_to_status(data: &Data, point: Point) -> PointStatus {
    // println!("point_to_status: point={point:?}");

    let result = data
        .iter()
        .filter(|(s, b)| s.distance(&point) <= s.distance(b))
        .map(|(s, b)| {
            if point == *s {
                PointStatus::Sensor
            } else if point == *b {
                PointStatus::Beacon
            } else {
                PointStatus::Covered
            }
        })
        .fold(PointStatus::Uncovered, |acc, status| acc.max(status));

    // println!("point_to_status: result={result:?}");
    result
}
