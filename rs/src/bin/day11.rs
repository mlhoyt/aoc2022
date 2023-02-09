use aoc;
use std::collections::VecDeque;

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{}", input);

    let data = parse(&input).expect("cannot parse input");
    // for v in data.iter() {
    //     println!("{:?}", v);
    // }

    let result1 = part1(&data);
    println!("part1: {}", result1);

    // let result2 = part2(&data);
    // println!("part2: {}", result2);
}

fn parse(input: &str) -> Result<Data, String> {
    input
        .split("\n\n")
        .map(|s| s.parse::<Monkey>())
        .collect::<Result<_, _>>()
}

type Data = Vec<Monkey>;

#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    inspect_operation: InspectOperation,
    inspect_value: InspectValue,
    test_value: u128,
    test_on_true: usize,
    test_on_false: usize,
}

#[derive(Debug)]
enum InspectOperation {
    Add,
    Multiply,
}

#[derive(Debug)]
enum InspectValue {
    Literal(u128),
    Old,
}

impl std::str::FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items: Vec<u128> = vec![];
        let mut has_items = false;
        let mut inspect_operation = InspectOperation::Add;
        let mut has_inspect_operation = false;
        let mut inspect_value = InspectValue::Old;
        let mut has_inspect_value = false;
        let mut test_value: u128 = 0;
        let mut has_test_value = false;
        let mut test_on_true: usize = 0;
        let mut has_test_on_true = false;
        let mut test_on_false: usize = 1;
        let mut has_test_on_false = false;

        for l in s.lines() {
            let mut fields = l.split_whitespace();
            match fields.next() {
                Some("Monkey") => {
                    // do nothing
                }
                Some("Starting") => {
                    match fields.next() {
                        Some("items:") => {
                            let item_list: Result<Vec<u128>, _> = fields
                                .collect::<String>()
                                .split::<&str>(",".into())
                                .map(|s| s.parse::<u128>())
                                .collect::<Result<_, _>>();
                            if item_list.is_err() {
                                return Err("failed parsing starting items list".into());
                            }
                            items.append(&mut item_list.unwrap());
                            has_items = true;
                        }
                        _ => {
                            return Err("unpexted starting items input".into());
                        }
                    };
                }
                Some("Operation:") => {
                    match fields.next() {
                        Some("new") => {
                            match fields.next() {
                                Some("=") => {
                                    match fields.next() {
                                        Some("old") => {
                                            match fields.next() {
                                                Some("+") => {
                                                    inspect_operation = InspectOperation::Add;
                                                    has_inspect_operation = true;
                                                    match fields.next() {
                                                        Some("old") => {
                                                            inspect_value = InspectValue::Old;
                                                            has_inspect_value = true;
                                                        }
                                                        Some(v) => {
                                                            if let Ok(vn) = v.parse() {
                                                                inspect_value =
                                                                    InspectValue::Literal(vn);
                                                                has_inspect_value = true;
                                                            } else {
                                                                return Err("failed parseing operation argument as number".into());
                                                            }
                                                        }
                                                        _ => {
                                                            return Err("operation input requires one argument".into());
                                                        }
                                                    };
                                                }
                                                Some("*") => {
                                                    inspect_operation = InspectOperation::Multiply;
                                                    has_inspect_operation = true;
                                                    match fields.next() {
                                                        Some("old") => {
                                                            inspect_value = InspectValue::Old;
                                                            has_inspect_value = true;
                                                        }
                                                        Some(v) => {
                                                            if let Ok(vn) = v.parse() {
                                                                inspect_value =
                                                                    InspectValue::Literal(vn);
                                                                has_inspect_value = true;
                                                            } else {
                                                                return Err("failed parseing operation argument as number".into());
                                                            }
                                                        }
                                                        _ => {
                                                            return Err("operation input requires one argument".into());
                                                        }
                                                    };
                                                }
                                                _ => {
                                                    return Err("unexpected operation input".into());
                                                }
                                            };
                                        }
                                        _ => {
                                            return Err("unpexted operation input".into());
                                        }
                                    };
                                }
                                _ => {
                                    return Err("unpexted operation input".into());
                                }
                            };
                        }
                        _ => {
                            return Err("unpexted operation input".into());
                        }
                    };
                }
                Some("Test:") => {
                    match fields.next() {
                        Some("divisible") => {
                            match fields.next() {
                                Some("by") => {
                                    match fields.next() {
                                        Some(v) => {
                                            if let Ok(vn) = v.parse() {
                                                test_value = vn;
                                                has_test_value = true;
                                            } else {
                                                return Err("failed parsing test expression argument as number".into());
                                            }
                                        }
                                        _ => {
                                            return Err("unpexted test expression input".into());
                                        }
                                    };
                                }
                                _ => {
                                    return Err("unpexted test expression input".into());
                                }
                            };
                        }
                        _ => {
                            return Err("unpexted test expression input".into());
                        }
                    };
                }
                Some("If") => {
                    match fields.next() {
                        Some("true:") => {
                            match fields.next() {
                                Some("throw") => {
                                    match fields.next() {
                                        Some("to") => {
                                            match fields.next() {
                                                Some("monkey") => {
                                                    match fields.next() {
                                                        Some(v) => {
                                                            if let Ok(vn) = v.parse() {
                                                                test_on_true = vn;
                                                                has_test_on_true = true;
                                                            } else {
                                                                return Err("failed parsing test branch argument as number".into());
                                                            }
                                                        }
                                                        _ => {
                                                            return Err(
                                                                "unpexted test expression input"
                                                                    .into(),
                                                            );
                                                        }
                                                    };
                                                }
                                                _ => {
                                                    return Err(
                                                        "unexpected test branch input".into()
                                                    );
                                                }
                                            };
                                        }
                                        _ => {
                                            return Err("unexpected test branch input".into());
                                        }
                                    };
                                }
                                _ => {
                                    return Err("unexpected test branch input".into());
                                }
                            };
                        }
                        Some("false:") => {
                            match fields.next() {
                                Some("throw") => {
                                    match fields.next() {
                                        Some("to") => {
                                            match fields.next() {
                                                Some("monkey") => {
                                                    match fields.next() {
                                                        Some(v) => {
                                                            if let Ok(vn) = v.parse() {
                                                                test_on_false = vn;
                                                                has_test_on_false = true;
                                                            } else {
                                                                return Err("failed parsing test branch argument as number".into());
                                                            }
                                                        }
                                                        _ => {
                                                            return Err(
                                                                "unpexted test expression input"
                                                                    .into(),
                                                            );
                                                        }
                                                    };
                                                }
                                                _ => {
                                                    return Err(
                                                        "unexpected test branch input".into()
                                                    );
                                                }
                                            };
                                        }
                                        _ => {
                                            return Err("unexpected test branch input".into());
                                        }
                                    };
                                }
                                _ => {
                                    return Err("unexpected test branch input".into());
                                }
                            };
                        }
                        _ => {
                            return Err("unexpected test branch input".into());
                        }
                    };
                }
                _ => {
                    return Err("unexpected input".into());
                }
            };
        }

        if has_items
            && has_inspect_operation
            && has_inspect_value
            && has_test_value
            && has_test_on_true
            && has_test_on_false
        {
            Ok(Monkey {
                items,
                inspect_operation,
                inspect_value,
                test_value,
                test_on_true,
                test_on_false,
            })
        } else {
            Err("parsing completed but some fields are missing".into())
        }
    }
}

fn part1(data: &Data) -> u128 {
    let mut items: Vec<_> = data
        .iter()
        .map(|v| v.items.iter().cloned().collect::<VecDeque<_>>())
        .collect();
    let mut inspect_counts = vec![0; items.len()];

    for _ in 0..20 {
        (items, inspect_counts) = simulate_round(&data, &items, &inspect_counts);
    }

    inspect_counts.sort_by(|a, b| b.cmp(a));
    inspect_counts.iter().take(2).product()
}

// This does NOT yield the correct solution.
// See the note in simulate_round under Relief part2 for an explanation why.
fn part2(data: &Data) -> u128 {
    let mut items: Vec<_> = data
        .iter()
        .map(|v| v.items.iter().cloned().collect::<VecDeque<_>>())
        .collect();
    let mut inspect_counts = vec![0; items.len()];

    for _ in 0..10000 {
        (items, inspect_counts) = simulate_round(&data, &items, &inspect_counts);
    }

    inspect_counts.sort_by(|a, b| b.cmp(a));
    inspect_counts.iter().take(2).product()
}

fn simulate_round(
    data: &Data,
    items: &Vec<VecDeque<u128>>,
    inspect_counts: &Vec<u128>,
) -> (Vec<VecDeque<u128>>, Vec<u128>) {
    let mut next_items = items.clone();
    let mut next_inspect_counts = inspect_counts.clone();

    for i in 0..next_items.len() {
        while let Some(old) = next_items[i].pop_front() {
            let mut new = old;
            // println!(
            //     "Monkey {} inspects an item with a worry level of {}",
            //     i, new
            // );

            // Inspect
            next_inspect_counts[i] += 1;
            match data[i].inspect_operation {
                InspectOperation::Add => {
                    new += match data[i].inspect_value {
                        InspectValue::Literal(n) => n,
                        InspectValue::Old => old,
                    };
                }
                InspectOperation::Multiply => {
                    new *= match data[i].inspect_value {
                        InspectValue::Literal(n) => n,
                        InspectValue::Old => old,
                    };
                }
            };
            // println!(
            //     "- Worry level increases (by {:?} {:?}) to {}",
            //     data[i].inspect_operation, data[i].inspect_value, new
            // );

            // Relief
            // part2: Instead of the opaque `floor(x/3)` this is supposed to use
            // [Chinese Remainder Theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem)
            // which means `x%n` where `n` is product of all the `data[].test_value`.  This CRT has
            // come up before in AoC and although I recognized for both inputs that all the
            // `data[].test_value` were prime meaning `gcm(...) = product(...)`, I neither
            // understand the math behind the CRT nor, and perhaps more importantly, how to be sure
            // it applies so coding that solution has a feel of cheating I cannot stomach.  And,
            // ultimately, I don't want the star that bad.
            new = (new as f64 / 3.0).floor() as u128;
            // println!(
            //     "-- Monkey gets bored with item. Worry level is divided by 3 to {}",
            //     new
            // );

            // Decide
            let ip = if new % data[i].test_value == 0 {
                // println!(
                //     "--- Current worry level IS divisible by {}",
                //     data[i].test_value
                // );
                data[i].test_on_true
            } else {
                // println!(
                //     "--- Current worry level is NOT divisible by {}",
                //     data[i].test_value
                // );
                data[i].test_on_false
            };

            // Throw
            // println!(
            //     "---- Item with worry level {} is thrown to monkey {}",
            //     new, ip
            // );
            next_items[ip].push_back(new);
        }
    }

    (next_items, next_inspect_counts)
}
