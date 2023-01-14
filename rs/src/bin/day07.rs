use aoc;

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{}", input);

    let data = parse(&input).expect("cannot parse input");
    // println!("dir-count={}", data.len());
    // for (d, s) in &data {
    //     println!("dir={:?} size={}", d, s);
    // }

    let result1 = part1(&data);
    println!("part1: {}", result1);

    let result2 = part2(&data);
    println!("part2: {}", result2);
}

fn parse(input: &str) -> Result<Data, String> {
    let mut pwd = vec!["/".to_string()];
    let mut dirs = Data::new();

    let pwd_to_dirs = |path: &[String]| -> Vec<Vec<String>> {
        (0..path.len()).map(|i| path[..=i].to_vec()).collect()
    };

    input.lines().for_each(|l| {
        if l.starts_with("$ cd ") {
            let dir = l.split_whitespace().last().unwrap();
            if dir == ".." {
                pwd.pop();
            } else if dir == "/" {
                pwd = vec!["/".to_string()];
            } else {
                pwd.push(dir.to_string());
            }
        } else if l.starts_with("$ ls") {
            // do nothing
        } else if l.starts_with("dir ") {
            // do nothing
        } else {
            let file_attrs: Vec<_> = l.split_whitespace().collect(); // \d+ \s+
            let file_size: usize = file_attrs[0].parse().expect("cannot parse file size");

            pwd_to_dirs(&pwd).into_iter().for_each(|dir| {
                let dir_size = dirs.entry(dir.clone()).or_insert(0);
                *dir_size += file_size;
            });
        }
    });

    Ok(dirs)
}

type Data = std::collections::HashMap<Vec<String>, usize>;

fn part1(data: &Data) -> usize {
    data.iter()
        .map(|(_, s)| if *s <= 100_000 { *s } else { 0 })
        .sum()
}

fn part2(data: &Data) -> usize {
    let used = data.get(&vec!["/".to_string()]).unwrap();
    let free = 70_000_000 - *used;
    let to_free = 30_000_000 - free;

    data.iter().fold(
        70_000_000,
        |acc, (_, s)| {
            if *s < acc && *s > to_free {
                *s
            } else {
                acc
            }
        },
    )
}

// The solution naturally is a tree structure but trees, like linked lists, are challenging in Rust
// because of the ownership model.  reference-counter (Rc) provide a means for this in Rust.
//
// For this problem:
// - The "parent" field is valuable as we will be traversing backwards.
// - I don't know if RefCell will be necessary given what I have done elsewhere with Rc::make_mut
//   but I could be wrong and RefCell might just be easier / more straight-forward.
//
// See: https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75
// use std::rc:Rc;
// let ref = Rc::new(value); // moves 'value' into Rc
// let clone1 = Rc::clone(&ref); // create a clone of a reference
// let clone2 = Rc::clone(&ref); // creates another clone
//
// struct TreeNode {
//   value: Option<FIXME>,
//   children: Vec<Rc<RefCell<TreeNode>>>,
//   parent: Option<Rc<RefCell<TreeNode>>>,
// }
