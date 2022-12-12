use clap::Parser;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn part_a(input: String) -> i32 {
    let (mut start_row, mut start_col) = (0, 0);
    let (mut end_row, mut end_col) = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .into_iter()
                .enumerate()
                .map(|(col, x)| {
                    if *x == b'S' {
                        start_row = row;
                        start_col = col;
                        0
                    } else if *x == b'E' {
                        end_row = row;
                        end_col = col;
                        25
                    } else {
                        x - b'a'
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut queue = VecDeque::new();
    queue.push_back((start_row, start_col));

    let mut visited = HashSet::new();
    visited.insert((start_row, start_col));

    let mut steps = 0;
    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let (row, col) = queue.pop_front().unwrap();
            if row == end_row && col == end_col {
                return steps;
            }

            let row = row as isize;
            let col = col as isize;

            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let r = row + dr;
                let c = col + dc;

                if r >= 0 && r < grid.len() as isize && c >= 0 && c < grid[0].len() as isize {
                    let r = r as usize;
                    let c = c as usize;
                    if grid[r][c] <= grid[row as usize][col as usize] + 1
                        && !visited.contains(&(r, c))
                    {
                        visited.insert((r, c));
                        queue.push_back((r, c));
                    }
                }
            }
        }

        steps += 1;
    }

    0
}

fn part_b(input: String) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let (mut end_row, mut end_col) = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .into_iter()
                .enumerate()
                .map(|(col, x)| {
                    if *x == b'a' || *x == b'S' {
                        queue.push_back((row, col));
                        visited.insert((row, col));
                        0
                    } else if *x == b'E' {
                        end_row = row;
                        end_col = col;
                        25
                    } else {
                        x - b'a'
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut steps = 0;
    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let (row, col) = queue.pop_front().unwrap();
            if row == end_row && col == end_col {
                return steps;
            }

            let row = row as isize;
            let col = col as isize;

            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let r = row + dr;
                let c = col + dc;

                if r >= 0 && r < grid.len() as isize && c >= 0 && c < grid[0].len() as isize {
                    let r = r as usize;
                    let c = c as usize;
                    if grid[r][c] <= grid[row as usize][col as usize] + 1
                        && !visited.contains(&(r, c))
                    {
                        visited.insert((r, c));
                        queue.push_back((r, c));
                    }
                }
            }
        }

        steps += 1;
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
