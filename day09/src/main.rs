use clap::Parser;
use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn part_a(input: String) -> i32 {
    let (mut head_row, mut head_col) = (0, 0);
    let (mut tail_row, mut tail_col) = (0, 0);

    let mut set = HashSet::new();
    set.insert((0, 0));

    for line in input.lines() {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

        let distance = parts[1].parse::<i32>().unwrap();

        for _ in 0..distance {
            let (mut dr, mut dc) = (0, 0);
            match parts[0].as_bytes()[0] {
                b'R' => dc = 1,
                b'L' => dc = -1,
                b'D' => dr = 1,
                b'U' => dr = -1,
                _ => unreachable!(),
            }

            head_row += dr;
            head_col += dc;

            println!("{},{}", tail_row - head_row, tail_col - head_col);

            match (tail_row - head_row, tail_col - head_col) {
                (2, 0) => tail_row = head_row + 1,
                (2, 1) => {
                    tail_row = head_row + 1;
                    tail_col = head_col;
                }
                (2, -1) => {
                    tail_row = head_row + 1;
                    tail_col = head_col;
                }
                (-2, 0) => tail_row = head_row - 1,
                (-2, 1) => {
                    tail_row = head_row - 1;
                    tail_col = head_col;
                }
                (-2, -1) => {
                    tail_row = head_row - 1;
                    tail_col = head_col;
                }
                (0, 2) => tail_col = head_col + 1,
                (1, 2) => {
                    tail_col = head_col + 1;
                    tail_row = head_row;
                }
                (-1, 2) => {
                    tail_col = head_col + 1;
                    tail_row = head_row;
                }
                (0, -2) => tail_col = head_col - 1,
                (1, -2) => {
                    tail_col = head_col - 1;
                    tail_row = head_row;
                }
                (-1, -2) => {
                    tail_col = head_col - 1;
                    tail_row = head_row;
                }
                _ => println!("!!!{},{}", tail_row - head_row, tail_col - head_col),
            }

            // println!("{}:{}", tail_row, tail_col);
            // println!("=={}:{}", head_row, head_col);

            set.insert((tail_row, tail_col));
        }
    }

    set.len() as i32
}

fn part_b(input: String) -> i32 {
    let mut positions = vec![];
    positions.resize(10, (0, 0));

    let mut set = HashSet::new();
    set.insert((0, 0));

    for line in input.lines() {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

        let distance = parts[1].parse::<i32>().unwrap();

        for _ in 0..distance {
            let (mut dr, mut dc) = (0, 0);
            match parts[0].as_bytes()[0] {
                b'R' => dc = 1,
                b'L' => dc = -1,
                b'D' => dr = 1,
                b'U' => dr = -1,
                _ => unreachable!(),
            }

            positions[0].0 += dr;
            positions[0].1 += dc;

            for i in 0..positions.len() - 1 {
                let (head_row, head_col) = positions[i];
                let (mut tail_row, mut tail_col) = positions[i + 1];

                match (tail_row - head_row, tail_col - head_col) {
                    (2, 0) => tail_row = head_row + 1,
                    (2, 1) => {
                        tail_row = head_row + 1;
                        tail_col = head_col;
                    }
                    (2, 2) => {
                        tail_row = head_row + 1;
                        tail_col = head_col + 1;
                    }
                    (2, -1) => {
                        tail_row = head_row + 1;
                        tail_col = head_col;
                    }
                    (-2, 0) => tail_row = head_row - 1,
                    (-2, 1) => {
                        tail_row = head_row - 1;
                        tail_col = head_col;
                    }
                    (-2, 2) => {
                        tail_row = head_row - 1;
                        tail_col = head_col + 1;
                    }
                    (-2, -1) => {
                        tail_row = head_row - 1;
                        tail_col = head_col;
                    }
                    (0, 2) => tail_col = head_col + 1,
                    (1, 2) => {
                        tail_col = head_col + 1;
                        tail_row = head_row;
                    }
                    (2, -2) => {
                        tail_row = head_row + 1;
                        tail_col = head_col - 1;
                    }
                    (-1, 2) => {
                        tail_col = head_col + 1;
                        tail_row = head_row;
                    }
                    (0, -2) => tail_col = head_col - 1,
                    (1, -2) => {
                        tail_col = head_col - 1;
                        tail_row = head_row;
                    }
                    (-2, -2) => {
                        tail_row = head_row - 1;
                        tail_col = head_col - 1;
                    }
                    (-1, -2) => {
                        tail_col = head_col - 1;
                        tail_row = head_row;
                    }
                    (0, 0) | (1, 0) | (0, 1) | (-1, 0) | (0, -1) => {}
                    _ => println!("!!!{},{}", tail_row - head_row, tail_col - head_col),
                }

                positions[i + 1] = (tail_row, tail_col);
            }

            set.insert(positions.last().unwrap().clone());
        }
    }

    set.len() as i32
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
