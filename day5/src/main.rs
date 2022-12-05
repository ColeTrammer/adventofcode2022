use clap::Parser;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn part_a(input: String) -> String {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let mut pic = parts[0].lines().collect::<Vec<_>>();
    let ll = pic.pop().unwrap();
    let len = ll.split(" ").filter(|x| !x.is_empty()).count();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; len];

    for line in pic {
        let line = line.replace("    ", " [_] ");
        line.split(" ")
            .filter(|x| !x.is_empty())
            .enumerate()
            .for_each(|(i, mut x)| {
                if x == "[_]" {
                    return;
                }
                x = x.strip_prefix("[").unwrap();
                x = x.strip_suffix("]").unwrap();
                let y = x.parse::<char>().unwrap();

                stacks[i].insert(0, y);
            });
    }

    let ins = parts[1];
    for i in ins.lines() {
        let p = i.split(" ").collect::<Vec<_>>();
        let a = p[1].parse::<usize>().unwrap();
        let b = p[3].parse::<usize>().unwrap();
        let c = p[5].parse::<usize>().unwrap();

        for _ in 0..a {
            if let Some(x) = stacks[b - 1].pop() {
                stacks[c - 1].push(x);
            }
        }
    }

    stacks.into_iter().map(|x| *x.last().unwrap()).collect()
}

fn part_b(input: String) -> String {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let mut pic = parts[0].lines().collect::<Vec<_>>();
    let ll = pic.pop().unwrap();
    let len = ll.split(" ").filter(|x| !x.is_empty()).count();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; len];

    for line in pic {
        let line = line.replace("    ", " [_] ");
        line.split(" ")
            .filter(|x| !x.is_empty())
            .enumerate()
            .for_each(|(i, mut x)| {
                if x == "[_]" {
                    return;
                }
                x = x.strip_prefix("[").unwrap();
                x = x.strip_suffix("]").unwrap();
                let y = x.parse::<char>().unwrap();

                stacks[i].insert(0, y);
            });
    }

    let ins = parts[1];
    for i in ins.lines() {
        let p = i.split(" ").collect::<Vec<_>>();
        let a = p[1].parse::<usize>().unwrap();
        let b = p[3].parse::<usize>().unwrap();
        let c = p[5].parse::<usize>().unwrap();

        let mut s = (0..a)
            .map(|_| stacks[b - 1].pop().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>();

        stacks[c - 1].append(&mut s);
    }

    stacks.into_iter().map(|x| *x.last().unwrap()).collect()
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

    if args.part_b {
        println!("Part B: {:?}", part_b(input.clone()));
    } else {
        println!("Part A: {:?}", part_a(input.clone()));
    }

    Ok(())
}
