use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug)]
enum Op {
    Plus,
    Minus,
    Mult,
    Div,
    Yell(i64),
}

fn part_a(input: String) -> i64 {
    let mut nodes = HashMap::new();

    for line in input.lines() {
        let parts = line.split(&[' ', ':']).collect::<Vec<_>>();
        println!("{:?}", parts);

        let name = parts[0];
        if parts.len() == 3 {
            nodes.insert(name, ("", "", Op::Yell(parts[2].parse().unwrap())));
        } else {
            let op = match parts[3].as_bytes()[0] {
                b'+' => Op::Plus,
                b'-' => Op::Minus,
                b'*' => Op::Mult,
                b'/' => Op::Div,
                _ => Op::Yell(0),
            };

            nodes.insert(name, (parts[2], parts[4], op));
        }
    }

    let mut sorted = vec![];
    let mut perm_marks = HashSet::new();
    let mut temp_marks = HashSet::new();

    fn visit<'a>(
        node: &'a str,
        nodes: &HashMap<&'a str, (&'a str, &'a str, Op)>,
        sorted: &mut Vec<&'a str>,
        perm_marks: &mut HashSet<&'a str>,
        temp_marks: &mut HashSet<&'a str>,
    ) {
        if perm_marks.contains(node) {
            return;
        }
        if temp_marks.contains(node) {
            println!("Has cycle!");
            return;
        }

        temp_marks.insert(node);

        let left = nodes.get(node).unwrap().0;
        let right = nodes.get(node).unwrap().1;
        if !left.is_empty() {
            visit(left, nodes, sorted, perm_marks, temp_marks);
        }
        if !right.is_empty() {
            visit(right, nodes, sorted, perm_marks, temp_marks);
        }

        temp_marks.remove(node);
        perm_marks.insert(node);
        sorted.push(node);
    }

    visit(
        "root",
        &nodes,
        &mut sorted,
        &mut perm_marks,
        &mut temp_marks,
    );

    let mut results = HashMap::new();
    for node in sorted {
        let (left, right, op) = nodes.get(node).cloned().unwrap();
        let result = match op {
            Op::Plus => *results.get(left).unwrap() + *results.get(right).unwrap(),
            Op::Minus => *results.get(left).unwrap() - *results.get(right).unwrap(),
            Op::Mult => *results.get(left).unwrap() * *results.get(right).unwrap(),
            Op::Div => *results.get(left).unwrap() / *results.get(right).unwrap(),
            Op::Yell(n) => n,
        };
        results.insert(node, result);
    }

    *results.get("root").unwrap()
}

fn part_b(input: String) -> i64 {
    let mut nodes = HashMap::new();

    for line in input.lines() {
        let parts = line.split(&[' ', ':']).collect::<Vec<_>>();
        println!("{:?}", parts);

        let name = parts[0];
        if parts.len() == 3 {
            nodes.insert(name, ("", "", Op::Yell(parts[2].parse().unwrap())));
        } else {
            let op = match parts[3].as_bytes()[0] {
                b'+' => Op::Plus,
                b'-' => Op::Minus,
                b'*' => Op::Mult,
                b'/' => Op::Div,
                _ => Op::Yell(0),
            };

            nodes.insert(name, (parts[2], parts[4], op));
        }
    }

    let mut perm_marks = HashSet::new();
    let mut temp_marks = HashSet::new();

    fn visit<'a>(
        node: &'a str,
        nodes: &HashMap<&'a str, (&'a str, &'a str, Op)>,
        sorted: &mut Vec<&'a str>,
        perm_marks: &mut HashSet<&'a str>,
        temp_marks: &mut HashSet<&'a str>,
    ) {
        if perm_marks.contains(node) {
            return;
        }
        if temp_marks.contains(node) {
            println!("Has cycle!");
            return;
        }

        temp_marks.insert(node);

        let left = nodes.get(node).unwrap().0;
        let right = nodes.get(node).unwrap().1;
        if !left.is_empty() {
            visit(left, nodes, sorted, perm_marks, temp_marks);
        }
        if !right.is_empty() {
            visit(right, nodes, sorted, perm_marks, temp_marks);
        }

        temp_marks.remove(node);
        perm_marks.insert(node);
        sorted.push(node);
    }

    let mut sorted = vec![];

    visit(
        "root",
        &nodes,
        &mut sorted,
        &mut perm_marks,
        &mut temp_marks,
    );

    let mut results = HashMap::new();
    nodes.get_mut("humn").unwrap().2 = Op::Yell(3952673930912);
    for node in sorted {
        let (left, right, op) = nodes.get(node).cloned().unwrap();
        let result = match op {
            Op::Plus => *results.get(left).unwrap() + *results.get(right).unwrap(),
            Op::Minus => *results.get(left).unwrap() - *results.get(right).unwrap(),
            Op::Mult => *results.get(left).unwrap() * *results.get(right).unwrap(),
            Op::Div => *results.get(left).unwrap() / *results.get(right).unwrap(),
            Op::Yell(n) => n,
        };
        results.insert(node, result);
    }

    let (left, right, _) = nodes.get("root").cloned().unwrap();

    let lv = *results.get(left).unwrap();
    let rv = *results.get(right).unwrap();

    println!("{} = {}", lv, rv);

    fn contains_humn<'a>(node: &'a str, nodes: &HashMap<&'a str, (&'a str, &'a str, Op)>) -> bool {
        if node.is_empty() {
            return false;
        }

        if node == "humn" {
            return true;
        }

        let (left, right, _) = nodes.get(node).cloned().unwrap();
        contains_humn(left, nodes) || contains_humn(right, nodes)
    }

    fn find_value<'a>(
        node: &'a str,
        nodes: &HashMap<&'a str, (&'a str, &'a str, Op)>,
        results: &HashMap<&'a str, i64>,
        target: i64,
    ) -> i64 {
        if node == "humn" {
            return target;
        }

        let (left, right, op) = nodes.get(node).cloned().unwrap();

        let left_has_humn = contains_humn(left, nodes);
        let right_has_humn = contains_humn(right, nodes);

        assert!(left_has_humn || right_has_humn);
        assert!(!(left_has_humn && right_has_humn));

        if node == "root" {
            if left_has_humn {
                find_value(left, nodes, results, *results.get(right).unwrap())
            } else {
                find_value(right, nodes, results, *results.get(left).unwrap())
            }
        } else {
            let new_target = if left_has_humn {
                match op {
                    Op::Plus => target - *results.get(right).unwrap(),
                    Op::Minus => target + *results.get(right).unwrap(),
                    Op::Mult => target / *results.get(right).unwrap(),
                    Op::Div => target * *results.get(right).unwrap(),
                    _ => {
                        println!("{}", node);
                        unreachable!()
                    }
                }
            } else {
                // target = value + X
                //   X = target - value
                // target = value - X
                //   X = value - target
                // target = value * X
                //   X = value / target
                // target = value / X
                //   X = target / value
                match op {
                    Op::Plus => target - *results.get(left).unwrap(),
                    Op::Minus => *results.get(left).unwrap() - target,
                    Op::Mult => target / *results.get(left).unwrap(),
                    Op::Div => target / *results.get(left).unwrap(),
                    _ => {
                        println!("{}", node);
                        unreachable!()
                    }
                }
            };
            find_value(
                if left_has_humn { left } else { right },
                nodes,
                results,
                new_target,
            )
        }
    }

    find_value("root", &nodes, &results, 0)
}

#[derive(Parser, Debug)]
#[command(author)]
#[command(version)]
#[command(about = "Advent of Code 2022 Solution", long_about = None)]
struct Args {
    /// File to read input from
    #[arg(short, long, value_name = "FILE")]
    input: Option<PathBuf>,

    /// Whether to run part a or part b.
    #[arg(short = 'b', long, default_value_t = false)]
    part_b: bool,

    /// Whether to use test.txt or input.txt as the default file.
    #[arg(short = 't', long, default_value_t = false)]
    test: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let default = if args.test {
        "test.txt".into()
    } else {
        "input.txt".into()
    };

    let input = read_to_string(args.input.unwrap_or(default))?;

    if args.part_b {
        println!("Part B: {:?}", part_b(input.clone()));
    } else {
        println!("Part A: {:?}", part_a(input.clone()));
    }

    Ok(())
}
