use clap::Parser;
use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn to_priority(s: u8) -> i32 {
    if s >= b'a' && s <= b'z' {
        (s - b'a') as i32 + 1
    } else {
        (s - b'A') as i32 + 27
    }
}

fn part_a(runsacks: Vec<(&str, &str)>) -> i32 {
    runsacks
        .into_iter()
        .map(|(a, b)| {
            let set = a.as_bytes().into_iter().copied().collect::<HashSet<_>>();

            b.as_bytes()
                .into_iter()
                .copied()
                .collect::<HashSet<_>>()
                .into_iter()
                .map(|x| if set.contains(&x) { to_priority(x) } else { 0 })
                .sum::<i32>()
        })
        .sum()
}

fn part_b(groups: Vec<Vec<String>>) -> i32 {
    groups
        .into_iter()
        .map(|group| {
            let set1 = group[0].as_bytes().iter().copied().collect::<HashSet<_>>();
            let set2 = group[1].as_bytes().iter().copied().collect::<HashSet<_>>();
            for b in group[2].as_bytes() {
                if set1.contains(b) && set2.contains(b) {
                    return to_priority(*b);
                }
            }
            0
        })
        .sum()
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
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input = read_to_string(args.input.unwrap_or("input.txt".into()))?;

    let mut runsacks = vec![];
    for line in input.lines() {
        runsacks.push(line.split_at(line.len() / 2));
    }

    let groups = input
        .lines()
        .map(|x| x.to_owned())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();

    if args.part_b {
        println!("Part B: {:?}", part_b(groups.clone()));
    } else {
        println!("Part A: {:?}", part_a(runsacks.clone()));
    }

    Ok(())
}
