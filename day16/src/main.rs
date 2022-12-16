use clap::Parser;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn part_a(input: String) -> i32 {
    let mut nodes = HashMap::new();

    for line in input.lines() {
        let parts = line.split(&[',', '=', ';', ' ']).collect::<Vec<_>>();

        let node = parts[1];
        let rate = parts[5].parse::<i32>().unwrap();

        let mut edges = vec![];
        for i in (11..parts.len()).step_by(2) {
            edges.push(parts[i]);
        }

        nodes.insert(node, (rate, edges));
    }

    let mut useful_nodes = nodes
        .iter()
        .filter(|(_, (rate, _))| *rate > 0)
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();

    useful_nodes.push("AA");

    let mut lengths = HashMap::new();

    useful_nodes
        .iter()
        .cartesian_product(useful_nodes.iter())
        .filter(|(a, b)| *a != *b)
        .inspect(|(a, b)| {
            let mut length = 0;

            let mut queue = VecDeque::new();
            queue.push_back(**a);

            let mut visited = HashSet::new();
            visited.insert(**a);

            let mut found = false;
            while !queue.is_empty() && !found {
                let size = queue.len();
                for _ in 0..size {
                    let n = queue.pop_front().unwrap();
                    if n == **b {
                        found = true;
                        break;
                    }

                    if let Some((_, edges)) = nodes.get(n) {
                        for edge in edges {
                            if !visited.contains(*edge) {
                                queue.push_back(*edge);
                                visited.insert(*edge);
                            }
                        }
                    }
                }

                if found {
                    break;
                }
                length += 1;
            }

            lengths.entry(**a).or_insert(Vec::new()).push((**b, length));
        })
        .all(|_| true);

    let mut opened = HashSet::new();

    fn bt<'a>(
        nodes: &HashMap<&'a str, (i32, Vec<&'a str>)>,
        opened: &mut HashSet<&'a str>,
        lengths: &HashMap<&'a str, Vec<(&'a str, i32)>>,
        name: &'a str,
        time: i32,
    ) -> i32 {
        let m = 30;
        if time >= m {
            return 0;
        }

        let (rate, _) = nodes.get(name).unwrap();

        let rem = m - 1 - time;
        let score_for_opening = rate * rem;

        let mut best = if opened.contains(name) {
            0
        } else {
            score_for_opening
        };
        if !opened.contains(name) {
            opened.insert(name);
            for (edge, cost) in lengths.get(name).unwrap_or(&vec![]) {
                if *edge != "AA" && !opened.contains(*edge) {
                    best = best
                        .max(score_for_opening + bt(nodes, opened, lengths, edge, time + 1 + cost));
                }
            }
            opened.remove(name);
        }

        if name == "AA" {
            for (edge, cost) in lengths.get(name).unwrap_or(&vec![]) {
                if *edge != "AA" {
                    best = best.max(bt(nodes, opened, lengths, edge, time + cost));
                }
            }
        }

        best
    }

    bt(&nodes, &mut opened, &lengths, "AA", 0)
}

fn part_b(input: String) -> i32 {
    let mut nodes = HashMap::new();

    for line in input.lines() {
        let parts = line.split(&[',', '=', ';', ' ']).collect::<Vec<_>>();

        let node = parts[1];
        let rate = parts[5].parse::<i32>().unwrap();

        let mut edges = vec![];
        for i in (11..parts.len()).step_by(2) {
            edges.push(parts[i]);
        }

        nodes.insert(node, (rate, edges));
    }

    let mut useful_nodes = nodes
        .iter()
        .filter(|(_, (rate, _))| *rate > 0)
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();

    useful_nodes.push("AA");

    let mut lengths = HashMap::new();

    useful_nodes
        .iter()
        .cartesian_product(useful_nodes.iter())
        .filter(|(a, b)| *a != *b)
        .inspect(|(a, b)| {
            let mut length = 0;

            let mut queue = VecDeque::new();
            queue.push_back(**a);

            let mut visited = HashSet::new();
            visited.insert(**a);

            let mut found = false;
            while !queue.is_empty() && !found {
                let size = queue.len();
                for _ in 0..size {
                    let n = queue.pop_front().unwrap();
                    if n == **b {
                        found = true;
                        break;
                    }

                    if let Some((_, edges)) = nodes.get(n) {
                        for edge in edges {
                            if !visited.contains(*edge) {
                                queue.push_back(*edge);
                                visited.insert(*edge);
                            }
                        }
                    }
                }

                if found {
                    break;
                }
                length += 1;
            }

            lengths.entry(**a).or_insert(Vec::new()).push((**b, length));
        })
        .all(|_| true);

    let mut opened = HashSet::new();

    fn bt<'a>(
        nodes: &HashMap<&'a str, (i32, Vec<&'a str>)>,
        valid: &HashSet<&'a str>,
        opened: &mut HashSet<&'a str>,
        lengths: &HashMap<&'a str, Vec<(&'a str, i32)>>,
        name: &'a str,
        time: i32,
    ) -> i32 {
        let m = 26;
        if time >= m {
            return 0;
        }

        let (rate, _) = nodes.get(name).unwrap();

        let rem = m - 1 - time;
        let score_for_opening = rate * rem;

        let mut best = if opened.contains(name) {
            0
        } else {
            score_for_opening
        };
        if !opened.contains(name) {
            opened.insert(name);
            for (edge, cost) in lengths.get(name).unwrap_or(&vec![]) {
                if *edge != "AA" && !opened.contains(*edge) && valid.contains(*edge) {
                    best = best.max(
                        score_for_opening
                            + bt(nodes, valid, opened, lengths, edge, time + 1 + cost),
                    );
                }
            }
            opened.remove(name);
        }

        if name == "AA" {
            for (edge, cost) in lengths.get(name).unwrap_or(&vec![]) {
                if *edge != "AA" && valid.contains(*edge) {
                    best = best.max(bt(nodes, valid, opened, lengths, edge, time + cost));
                }
            }
        }

        best
    }

    let ruseful_nodes = useful_nodes
        .iter()
        .filter(|x| **x != "AA")
        .collect::<Vec<_>>();

    let mut best = 0;
    for i in 0..(1 << (ruseful_nodes.len() - 1)) {
        let other = !i;

        if i % 100 == 0 {
            println!("i={} m={}", i, (1 << (ruseful_nodes.len() - 1)));
        }

        let pa = ruseful_nodes
            .iter()
            .enumerate()
            .filter(|(ind, _)| ((1 << ind) & i) == 0)
            .map(|(_, x)| **x)
            .collect::<HashSet<_>>();

        let pb = ruseful_nodes
            .iter()
            .enumerate()
            .filter(|(ind, _)| ((1 << ind) & other) == 0)
            .map(|(_, x)| **x)
            .collect::<HashSet<_>>();

        best = best.max(
            bt(&nodes, &pa, &mut opened, &lengths, "AA", 0)
                + bt(&nodes, &pb, &mut opened, &lengths, "AA", 0),
        );
    }

    best
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
