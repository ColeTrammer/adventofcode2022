use clap::Parser;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn part_a(input: String) -> i32 {
    let grid = input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fn check_visibility(
        grid: &Vec<Vec<i32>>,
        row: usize,
        col: usize,
        delta: (isize, isize),
    ) -> bool {
        let mut row = row as isize;
        let mut col = col as isize;

        let start = grid[row as usize][col as usize];

        row += delta.0;
        col += delta.1;

        while !(row < 0 || col < 0 || row >= grid.len() as isize || col >= grid[0].len() as isize) {
            let x = grid[row as usize][col as usize];
            if x >= start {
                return false;
            }

            row += delta.0;
            col += delta.1;
        }

        true
    }

    let mut count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if check_visibility(&grid, row, col, (0, 1))
                || check_visibility(&grid, row, col, (0, -1))
                || check_visibility(&grid, row, col, (1, 0))
                || check_visibility(&grid, row, col, (-1, 0))
            {
                count += 1;
            }
        }
    }

    count
}

fn part_b(input: String) -> i32 {
    let grid = input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fn check_visibility(
        grid: &Vec<Vec<i32>>,
        row: usize,
        col: usize,
        delta: (isize, isize),
    ) -> i32 {
        let mut row = row as isize;
        let mut col = col as isize;

        let start = grid[row as usize][col as usize];

        row += delta.0;
        col += delta.1;

        let mut count = 0;

        while !(row < 0 || col < 0 || row >= grid.len() as isize || col >= grid[0].len() as isize) {
            count += 1;

            let x = grid[row as usize][col as usize];
            if x >= start {
                break;
            }

            row += delta.0;
            col += delta.1;
        }

        count
    }

    let mut scores = vec![];

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let score = check_visibility(&grid, row, col, (0, 1))
                * check_visibility(&grid, row, col, (0, -1))
                * check_visibility(&grid, row, col, (1, 0))
                * check_visibility(&grid, row, col, (-1, 0));
            scores.push(score);
        }
    }

    *scores.iter().max().unwrap()
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
