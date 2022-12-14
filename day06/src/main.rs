use clap::Parser;
use itertools::Itertools;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn part_a(input: String) -> usize {
    for (i, chunk) in input.as_bytes().windows(4).enumerate() {
        if chunk.iter().unique().count() == 4 {
            return i + 4;
        }
    }
    0
}

fn part_b(input: String) -> usize {
    for (i, chunk) in input.as_bytes().windows(14).enumerate() {
        if chunk.iter().unique().count() == 14 {
            return i + 14;
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
