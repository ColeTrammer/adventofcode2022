use clap::Parser;
use itertools::Itertools;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn part_a(input: String) -> usize {
    let mut grid = vec![vec![0; 1000]; 1000];

    for path in input.lines() {
        for (start, end) in path.split(" -> ").tuple_windows() {
            for (mut start_col, mut start_row) in start
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .tuple_windows()
            {
                for (end_col, end_row) in end
                    .split(",")
                    .map(|x| x.parse::<i32>().unwrap())
                    .tuple_windows()
                {
                    let mut dr = end_row - start_row;
                    if dr != 0 {
                        dr /= dr.abs();
                    }

                    let mut dc = end_col - start_col;
                    if dc != 0 {
                        dc /= dc.abs();
                    }

                    while start_row != end_row || start_col != end_col {
                        grid[start_row as usize][start_col as usize] = 1;
                        start_row += dr;
                        start_col += dc;
                    }
                    grid[start_row as usize][start_col as usize] = 1;
                }
            }
        }
    }

    loop {
        let mut row = 0;
        let mut col = 500;

        let mut exit = false;

        loop {
            // Got to the bottom.
            if row == 999 {
                exit = true;
                break;
            }

            if grid[row + 1][col] == 0 {
                row += 1;
            } else if col != 0 && grid[row + 1][col - 1] == 0 {
                row += 1;
                col -= 1;
            } else if col < 999 && grid[row + 1][col + 1] == 0 {
                row += 1;
                col += 1;
            } else {
                grid[row][col] = 2;
                println!("{},{}", row, col);
                break;
            }
        }

        let mut s = "".to_owned();
        for row in 0..=10 {
            for col in 490..=510 {
                if grid[row][col] == 0 {
                    s.push('.');
                } else if grid[row][col] == 1 {
                    s.push('#');
                } else {
                    s.push('O');
                }
            }
            s.push('\n');
        }
        println!("{}", s);
        if exit {
            break;
        }
    }

    grid.iter()
        .map(|x| x.iter().filter(|x| **x == 2).count())
        .sum()
}

fn part_b(input: String) -> usize {
    let mut max = 0;

    let mut grid = vec![vec![0; 1000]; 1000];

    for path in input.lines() {
        for (start, end) in path.split(" -> ").tuple_windows() {
            for (mut start_col, mut start_row) in start
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .tuple_windows()
            {
                for (end_col, end_row) in end
                    .split(",")
                    .map(|x| x.parse::<i32>().unwrap())
                    .tuple_windows()
                {
                    let mut dr = end_row - start_row;
                    if dr != 0 {
                        dr /= dr.abs();
                    }

                    let mut dc = end_col - start_col;
                    if dc != 0 {
                        dc /= dc.abs();
                    }

                    max = max.max(start_row);
                    max = max.max(end_row);

                    while start_row != end_row || start_col != end_col {
                        grid[start_row as usize][start_col as usize] = 1;
                        start_row += dr;
                        start_col += dc;
                    }
                    grid[start_row as usize][start_col as usize] = 1;
                }
            }
        }
    }

    for c in 0..1000 {
        grid[2 + max as usize][c] = 1;
    }

    loop {
        let mut row = 0;
        let mut col = 500;

        if grid[row][col] == 2 {
            break;
        }

        loop {
            if grid[row + 1][col] == 0 {
                row += 1;
            } else if col != 0 && grid[row + 1][col - 1] == 0 {
                row += 1;
                col -= 1;
            } else if col < 999 && grid[row + 1][col + 1] == 0 {
                row += 1;
                col += 1;
            } else {
                grid[row][col] = 2;
                println!("{},{}", row, col);
                break;
            }
        }

        // let mut s = "".to_owned();
        // for row in 0..=20 {
        //     for col in 485..=515 {
        //         if grid[row][col] == 0 {
        //             s.push('.');
        //         } else if grid[row][col] == 1 {
        //             s.push('#');
        //         } else {
        //             s.push('O');
        //         }
        //     }
        //     s.push('\n');
        // }
        // println!("{}", s);
    }

    grid.iter()
        .map(|x| x.iter().filter(|x| **x == 2).count())
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
