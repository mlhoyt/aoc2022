use aoc;
use std::collections::VecDeque;

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
    let cleaned_input = input.replace("\t", " ");
    let mut lines = cleaned_input.lines();

    let mut data: Data = (Vec::new(), Vec::new());

    // Process stack definitions
    while let Some(line) = lines.next() {
        if line.starts_with(" 1 ") {
            break;
        }

        // Process stacks
        let mut line_offset = 0;
        let mut stack_index = 0;
        while line_offset < line.len() {
            let chunk = &line[line_offset..];
            if chunk.len() < 3 {
                break;
            }

            // Initialize stack if necessary
            if data.0.len() < (stack_index + 1) {
                data.0.push(VecDeque::new());
            }

            // Push to stack
            if chunk.starts_with("[") {
                data.0[stack_index].push_back(chunk.chars().nth(1).unwrap());
            }

            // Update line-offset and queue index
            line_offset += 4;
            stack_index += 1;
        }
    }

    // Process empty separator line
    if let Some(line) = lines.next() {
        if !line.is_empty() {
            return Err("parsing failed expecting a blank line".into());
        }
    } else {
        return Err("parsing failed expecting a blank line".into());
    }

    // Process instructions
    while let Some(line) = lines.next() {
        let fields: Vec<_> = line.split_whitespace().collect();

        if fields.len() != 6 {
            return Err("parsing instruction yielded too few fields".into());
        }

        if fields[0] != "move" {
            return Err("parsing instruction yielded no move field".into());
        }

        let count: usize = match fields[1].parse() {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };

        if fields[2] != "from" {
            return Err("parsing instruction yielded no from field".into());
        }

        let from: usize = match fields[3].parse() {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };

        if fields[4] != "to" {
            return Err("parsing instruction yielded no to field".into());
        }

        let to: usize = match fields[5].parse() {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };

        // Push to list
        data.1.push(Instruction {
            count,
            from: from - 1,
            to: to - 1,
        });
    }

    Ok(data)
}

type Data = (Stacks, Instructions);

type Stacks = Vec<Stack>;

type Stack = VecDeque<char>;

type Instructions = Vec<Instruction>;

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn part1(data: &Data) -> String {
    data.1
        .iter()
        .fold(data.0.clone(), |acc, v| apply_instruction_9000(&acc, v))
        .into_iter()
        .map(|mut s| s.pop_front().unwrap())
        .collect()
}

fn apply_instruction_9000(data: &Stacks, instruction: &Instruction) -> Stacks {
    let mut next_data = data.clone();

    // Move one container at a time from front to front.
    for _ in 0..instruction.count {
        if let Some(v) = next_data[instruction.from].pop_front() {
            next_data[instruction.to].push_front(v);
        }
    }

    return next_data;
}

fn part2(data: &Data) -> String {
    data.1
        .iter()
        .fold(data.0.clone(), |acc, v| apply_instruction_9001(&acc, v))
        .into_iter()
        .map(|mut s| s.pop_front().unwrap())
        .collect()
}

fn apply_instruction_9001(data: &Stacks, instruction: &Instruction) -> Stacks {
    let mut next_data = data.clone();
    let mut tmp_stack = VecDeque::new();

    // Move one container at a time from front to front of the temporary stack.
    for _ in 0..instruction.count {
        if let Some(v) = next_data[instruction.from].pop_front() {
            tmp_stack.push_front(v);
        }
    }

    // Move one container at a time from front to front from the temporary stack.
    for _ in 0..instruction.count {
        if let Some(v) = tmp_stack.pop_front() {
            next_data[instruction.to].push_front(v);
        }
    }

    // By moving through a temporary stack the result is effectively moving all the containers at
    // once from one stack to another.
    // This reminds me of https://en.wikipedia.org/wiki/Tower_of_Hanoi

    return next_data;
}
