use clap::Parser;
use std::collections::{BTreeSet, HashSet};
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn part_a(input: String) -> i32 {
    let mut set = HashSet::new();

    // let target = 10;
    let target = 2000000;

    for line in input.lines() {
        let words = line.split(&[',', ' ']).collect::<Vec<_>>();
        println!("{:?}", words);

        let sx = words[2][2..].parse::<i32>().unwrap();
        let sy = words[4][2..words[4].len() - 1].parse::<i32>().unwrap();

        let bx = words[9][2..].parse::<i32>().unwrap();
        let by = words[11][2..].parse::<i32>().unwrap();

        let dist = (bx.abs_diff(sx)) + (by.abs_diff(sy));
        // println!("dist={}", dist);
        if let Some(dx) = dist.checked_sub(sy.abs_diff(target)) {
            // println!("dx={}", dx);
            for i in 0..=dx as i32 {
                if by != target || sx + i != bx {
                    // println!("add: {}", sx + i);
                    set.insert(sx + i);
                }
                if by != target || sx - i != bx {
                    // println!("add: {}", sx - i);
                    set.insert(sx - i);
                }
            }
        }
    }

    set.len() as i32
}

fn part_b(input: String) -> i64 {
    let mut parsed = vec![];

    let mut candidates = BTreeSet::new();
    let max = 4000000;
    // let max = 20;

    for line in input.lines() {
        let words = line.split(&[',', ' ']).collect::<Vec<_>>();
        // println!("{:?}", words);

        let sx = words[2][2..].parse::<i32>().unwrap();
        let sy = words[4][2..words[4].len() - 1].parse::<i32>().unwrap();

        let bx = words[9][2..].parse::<i32>().unwrap();
        let by = words[11][2..].parse::<i32>().unwrap();

        parsed.push((sx, sy, bx, by));

        // Add every point dist+1 to candidates.
        let dist = ((bx.abs_diff(sx)) + (by.abs_diff(sy)) + 1) as i32;
        let x = sx - dist;
        let x_end = sx + dist;
        for d in 0..dist {
            let mut add = |(x, y)| {
                if x >= 0 && x <= max && y >= 0 && y <= max {
                    candidates.insert((x, y));
                }
            };

            add((x + d, sy + d));
            add((x + d, sy - d));
            add((x_end + d, sy - d));
            add((x_end + d, sy - d));
        }
    }

    for (x, y) in candidates.into_iter() {
        let mut value = true;
        for (sx, sy, bx, by) in parsed.iter().cloned() {
            let known_distance = (bx.abs_diff(sx)) + (by.abs_diff(sy));
            let distance = (sx.abs_diff(x)) + (sy.abs_diff(y));

            // println!("d={distance} kd={known_distance}");

            if distance <= known_distance {
                value = false;
                break;
            }
        }

        if value {
            println!("x={x} y={y}");
            return (x as i64) * (max as i64) + (y as i64);
        }
    }

    0
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
