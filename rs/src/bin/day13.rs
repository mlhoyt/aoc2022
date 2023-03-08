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
    let pairs = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<PacketData>())
        .collect::<Result<Vec<PacketData>, String>>()?
        .chunks(2)
        .filter_map(|c| {
            if c.len() == 2 {
                Some((c[0].to_owned(), c[1].to_owned()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(Data { pairs })
}

#[derive(Debug)]
struct Data {
    pairs: Vec<(PacketData, PacketData)>,
}

#[derive(Debug, Clone, Eq)]
enum PacketData {
    Value(usize),
    List(Vec<PacketData>),
}

impl std::str::FromStr for PacketData {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') {
            if !s.ends_with(']') {
                return Err("packet-data list string must start with [ and end with ]".into());
            }

            let mut sp = s;
            sp = sp.strip_prefix('[').unwrap().strip_suffix(']').unwrap();

            let mut data: Vec<PacketData> = vec![];

            while !sp.is_empty() {
                if sp.starts_with('[') {
                    match find_to_matching_bracket(sp) {
                        Some(n) => {
                            let (packet_data, rest) = sp.split_at(n + 1);

                            match packet_data.parse::<PacketData>() {
                                Ok(v) => {
                                    data.push(v);
                                }
                                Err(e) => {
                                    return Err(e);
                                }
                            };

                            sp = rest;
                        }
                        _ => {
                            return Err(
                                "packet data list string must start with [ and end with ]".into()
                            );
                        }
                    };
                } else if sp.starts_with(char::is_numeric) {
                    let n = find_to_end_of_number(sp);
                    let (packet_data, rest) = sp.split_at(n + 1);

                    match packet_data.parse::<PacketData>() {
                        Ok(v) => data.push(v),
                        Err(e) => return Err(e),
                    };

                    sp = rest;
                } else {
                    return Err("unexpected packet data string".into());
                }

                if let Some(v) = sp.strip_prefix(',') {
                    sp = v
                }
            }

            Ok(PacketData::List(data))
        } else if s.starts_with(char::is_numeric) {
            match s.parse::<usize>() {
                Ok(n) => Ok(PacketData::Value(n)),
                _ => Err("failed to parse numeric packet-data entry".into()),
            }
        } else {
            return Err("unexpected packet-data string".into());
        }
    }
}

fn find_to_matching_bracket(s: &str) -> Option<usize> {
    let (_, n, matched): (usize, usize, bool) = s.chars().fold((0, 0, false), |(c, n, x), sn| {
        if !x {
            let cp = match sn {
                '[' => c + 1,
                ']' => c - 1,
                _ => c,
            };

            let (np, xp) = if cp == 0 { (n, true) } else { (n + 1, false) };

            (cp, np, xp)
        } else {
            (c, n, x)
        }
    });

    if matched {
        Some(n)
    } else {
        None
    }
}

// see https://fasterthanli.me/series/advent-of-code-2022/part-6
#[test]
fn test_find_to_matching_bracket() {
    assert_eq!(Some(1), find_to_matching_bracket("[]"));
    assert_eq!(Some(3), find_to_matching_bracket("[[]]"));
    assert_eq!(Some(5), find_to_matching_bracket("[1,[]]"));
    assert_eq!(Some(6), find_to_matching_bracket("[1,[2]]"));
    assert_eq!(Some(1), find_to_matching_bracket("[],[]"));
    assert_eq!(None, find_to_matching_bracket("["));
    assert_eq!(None, find_to_matching_bracket("[[]"));
    assert_eq!(None, find_to_matching_bracket("[1,[]"));
    assert_eq!(None, find_to_matching_bracket("[1,[2]"));
}

fn find_to_end_of_number(s: &str) -> usize {
    let (n, _) = s.chars().fold((-1, true), |(n, cont), sn| {
        if cont && sn.is_numeric() {
            (n + 1, true)
        } else {
            (n, false)
        }
    });

    n as usize
}

#[test]
fn test_find_to_end_of_number() {
    assert_eq!(0, find_to_end_of_number("1"));
    assert_eq!(1, find_to_end_of_number("12"));
    assert_eq!(1, find_to_end_of_number("12,"));
    assert_eq!(1, find_to_end_of_number("12,3"));
}

impl std::cmp::Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PacketData::Value(vl), PacketData::Value(vr)) => vl.cmp(vr),
            (PacketData::List(vl), PacketData::List(vr)) => {
                let lc = vl.iter().zip(vr.iter()).map(|(v1, v2)| v1.cmp(v2)).fold(
                    std::cmp::Ordering::Equal,
                    |acc, v| match (acc, v) {
                        (std::cmp::Ordering::Equal, v) => v,
                        (ne, _) => ne,
                    },
                );

                if lc != std::cmp::Ordering::Equal {
                    lc
                } else if vl.len() < vr.len() {
                    std::cmp::Ordering::Less
                } else if vl.len() > vr.len() {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
            (PacketData::Value(_), PacketData::List(_)) => {
                PacketData::List(vec![self.clone()]).cmp(other)
            }
            (PacketData::List(_), PacketData::Value(_)) => {
                self.cmp(&PacketData::List(vec![other.clone()]))
            }
        }
    }
}

#[test]
fn test_packetdata_cmp() {
    // PacketData::Value, PacketData::Value
    assert_eq!(
        std::cmp::Ordering::Equal,
        PacketData::Value(1).cmp(&PacketData::Value(1))
    );
    assert_eq!(
        std::cmp::Ordering::Less,
        PacketData::Value(1).cmp(&PacketData::Value(2))
    );
    assert_eq!(
        std::cmp::Ordering::Greater,
        PacketData::Value(2).cmp(&PacketData::Value(1))
    );

    // PacketData::List(vl), PacketData::List(vr)
    assert_eq!(
        std::cmp::Ordering::Equal,
        PacketData::List(vec![PacketData::Value(1)])
            .cmp(&PacketData::List(vec![PacketData::Value(1)]))
    );
    assert_eq!(
        std::cmp::Ordering::Less,
        PacketData::List(vec![PacketData::Value(1)])
            .cmp(&PacketData::List(vec![PacketData::Value(2)]))
    );
    assert_eq!(
        std::cmp::Ordering::Greater,
        PacketData::List(vec![PacketData::Value(2)])
            .cmp(&PacketData::List(vec![PacketData::Value(1)]))
    );
    assert_eq!(
        std::cmp::Ordering::Less,
        PacketData::List(vec![PacketData::Value(1)]).cmp(&PacketData::List(vec![
            PacketData::Value(1),
            PacketData::Value(1)
        ]))
    );
    assert_eq!(
        std::cmp::Ordering::Greater,
        PacketData::List(vec![PacketData::Value(1), PacketData::Value(1)])
            .cmp(&PacketData::List(vec![PacketData::Value(1)]))
    );

    // PacketData::Value(vl), PacketData::List(vr)
    assert_eq!(
        std::cmp::Ordering::Equal,
        PacketData::Value(1).cmp(&PacketData::List(vec![PacketData::Value(1)]))
    );
    assert_eq!(
        std::cmp::Ordering::Less,
        PacketData::Value(1).cmp(&PacketData::List(vec![PacketData::Value(2)]))
    );
    assert_eq!(
        std::cmp::Ordering::Greater,
        PacketData::Value(2).cmp(&PacketData::List(vec![PacketData::Value(1)]))
    );
    assert_eq!(
        std::cmp::Ordering::Less,
        PacketData::Value(1).cmp(&PacketData::List(vec![
            PacketData::Value(1),
            PacketData::Value(1)
        ]))
    );

    // PacketData::List(vl), PacketData::Value(vr)
    assert_eq!(
        std::cmp::Ordering::Equal,
        PacketData::List(vec![PacketData::Value(1)]).cmp(&PacketData::Value(1))
    );
    assert_eq!(
        std::cmp::Ordering::Less,
        PacketData::List(vec![PacketData::Value(1)]).cmp(&PacketData::Value(2))
    );
    assert_eq!(
        std::cmp::Ordering::Greater,
        PacketData::List(vec![PacketData::Value(2)]).cmp(&PacketData::Value(1))
    );
    assert_eq!(
        std::cmp::Ordering::Greater,
        PacketData::List(vec![PacketData::Value(1), PacketData::Value(1)])
            .cmp(&PacketData::Value(1))
    );
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PacketData::Value(vl), PacketData::Value(vr)) => vl == vr,
            (PacketData::List(vl), PacketData::List(vr)) => {
                if vl.len() == vr.len() {
                    vl.iter()
                        .zip(vr.iter())
                        .map(|(v1, v2)| *v1 == *v2)
                        .all(|v| v)
                } else {
                    false
                }
            }
            (PacketData::Value(_), PacketData::List(_)) => {
                *other == PacketData::List(vec![self.clone()])
            }
            (PacketData::List(_), PacketData::Value(_)) => {
                *self == PacketData::List(vec![other.clone()])
            }
        }
    }
}

#[test]
fn test_packetdata_eq() {
    // PacketData::Value, PacketData::Value
    assert_eq!(PacketData::Value(1), PacketData::Value(1));
    assert_ne!(PacketData::Value(1), PacketData::Value(2));

    // PacketData::List(vl), PacketData::List(vr)
    assert_eq!(
        PacketData::List(vec![PacketData::Value(1)]),
        PacketData::List(vec![PacketData::Value(1)])
    );
    assert_ne!(
        PacketData::List(vec![PacketData::Value(1)]),
        PacketData::List(vec![PacketData::Value(2)])
    );
    assert_ne!(
        PacketData::List(vec![PacketData::Value(1)]),
        PacketData::List(vec![PacketData::Value(1), PacketData::Value(2)])
    );

    // PacketData::Value(vl), PacketData::List(vr)
    assert_eq!(
        PacketData::Value(1),
        PacketData::List(vec![PacketData::Value(1)])
    );
    assert_ne!(
        PacketData::Value(2),
        PacketData::List(vec![PacketData::Value(1)])
    );
    assert_ne!(
        PacketData::Value(1),
        PacketData::List(vec![PacketData::Value(1), PacketData::Value(2)])
    );

    // PacketData::List(vl), PacketData::Value(vr)
    assert_eq!(
        PacketData::List(vec![PacketData::Value(1)]),
        PacketData::Value(1)
    );
    assert_ne!(
        PacketData::List(vec![PacketData::Value(1)]),
        PacketData::Value(2)
    );
    assert_ne!(
        PacketData::List(vec![PacketData::Value(1), PacketData::Value(2)]),
        PacketData::Value(1)
    );
}

fn part1(data: &Data) -> usize {
    data.pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (p1, p2))| if p1 < p2 { Some(i + 1) } else { None })
        .sum()
}

fn part2(data: &Data) -> usize {
    let mut all_packets: Vec<_> = data
        .pairs
        .iter()
        .flat_map(|(p1, p2)| vec![p1, p2])
        .collect();

    let div1 = "[[2]]".parse::<PacketData>().unwrap();
    let div2 = "[[6]]".parse::<PacketData>().unwrap();
    all_packets.append(&mut vec![&div1, &div2]);

    all_packets.sort();

    all_packets
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if **v == div1 || **v == div2 {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}
