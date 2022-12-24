use clap::Parser;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn simulate(
    (rows, cols): (usize, usize),
    (start_row, start_col): (usize, usize),
    (target_row, target_col): (usize, usize),
    acc: i32,
    mut blizzards: Vec<(usize, usize, i32)>,
) -> i32 {
    let mut queue = VecDeque::new();
    queue.push_back((start_row, start_col));

    let mut occupied_set = HashSet::new();

    let mut iter = acc;

    for _ in 0..acc {
        // Simulate blizzards.
        let mut new_blizzards = vec![];
        occupied_set.clear();

        for (row, col, dir) in blizzards.iter() {
            let (nr, nc) = match dir {
                0 => {
                    if col + 1 < cols - 1 {
                        (*row, *col + 1)
                    } else {
                        (*row, 1)
                    }
                }
                1 => {
                    if row + 1 < rows - 1 {
                        (*row + 1, *col)
                    } else {
                        (1, *col)
                    }
                }
                2 => {
                    if col - 1 > 0 {
                        (*row, *col - 1)
                    } else {
                        (*row, cols - 2)
                    }
                }
                3 => {
                    if row - 1 > 0 {
                        (*row - 1, *col)
                    } else {
                        (rows - 2, *col)
                    }
                }
                _ => unreachable!(),
            };

            new_blizzards.push((nr, nc, *dir));
            occupied_set.insert((nr, nc));
        }

        blizzards = new_blizzards;
    }

    let mut visited = HashSet::new();

    while !queue.is_empty() {
        // Simulate blizzards.
        let mut new_blizzards = vec![];
        occupied_set.clear();

        for (row, col, dir) in blizzards.iter() {
            let (nr, nc) = match dir {
                0 => {
                    if col + 1 < cols - 1 {
                        (*row, *col + 1)
                    } else {
                        (*row, 1)
                    }
                }
                1 => {
                    if row + 1 < rows - 1 {
                        (*row + 1, *col)
                    } else {
                        (1, *col)
                    }
                }
                2 => {
                    if col - 1 > 0 {
                        (*row, *col - 1)
                    } else {
                        (*row, cols - 2)
                    }
                }
                3 => {
                    if row - 1 > 0 {
                        (*row - 1, *col)
                    } else {
                        (rows - 2, *col)
                    }
                }
                _ => unreachable!(),
            };

            new_blizzards.push((nr, nc, *dir));
            occupied_set.insert((nr, nc));
        }

        blizzards = new_blizzards;

        let size = queue.len();
        for _ in 0..size {
            let (row, col) = queue.pop_front().unwrap();
            if row == target_row && col == target_col {
                return iter;
            }

            if !occupied_set.contains(&(row, col)) {
                queue.push_back((row, col));
            }

            for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (nr, nc) = (row + dr as usize, col + dc as usize);
                if !(nr > 0 && nr < rows - 1 && nc > 0 && nc < cols - 1)
                    && !(nr == target_row && nc == target_col)
                {
                    continue;
                }

                if !occupied_set.contains(&(nr, nc)) && !visited.contains(&(nr, nc, iter)) {
                    queue.push_back((nr, nc));
                    visited.insert((nr, nc, iter));
                }
            }
        }

        iter += 1;
    }

    0
}

fn part_a(input: String) -> i32 {
    let mut blizzards = vec![];

    let rows = input.lines().count();
    let mut cols = 0;
    for (row, line) in input.lines().enumerate() {
        cols = line.as_bytes().len();
        for (col, byte) in line.as_bytes().iter().enumerate() {
            let direction = match *byte {
                b'>' => 0,
                b'v' => 1,
                b'<' => 2,
                b'^' => 3,
                _ => 4,
            };
            if direction < 4 {
                blizzards.push((row, col, direction));
            }
        }
    }

    simulate(
        (rows, cols),
        (0, 1),
        (rows - 1, cols - 2),
        0,
        blizzards.clone(),
    )
}

fn part_b(input: String) -> i32 {
    let mut blizzards = vec![];

    let rows = input.lines().count();
    let mut cols = 0;
    for (row, line) in input.lines().enumerate() {
        cols = line.as_bytes().len();
        for (col, byte) in line.as_bytes().iter().enumerate() {
            let direction = match *byte {
                b'>' => 0,
                b'v' => 1,
                b'<' => 2,
                b'^' => 3,
                _ => 4,
            };
            if direction < 4 {
                blizzards.push((row, col, direction));
            }
        }
    }

    let a = simulate(
        (rows, cols),
        (0, 1),
        (rows - 1, cols - 2),
        0,
        blizzards.clone(),
    );

    let b = simulate(
        (rows, cols),
        (rows - 1, cols - 2),
        (0, 1),
        a,
        blizzards.clone(),
    );

    simulate(
        (rows, cols),
        (0, 1),
        (rows - 1, cols - 2),
        b,
        blizzards.clone(),
    )
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
