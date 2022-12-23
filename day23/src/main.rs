use clap::Parser;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn print(positions: &BTreeSet<(i32, i32)>) {
    let min_row = positions.iter().map(|(row, _)| *row).min().unwrap();
    let max_row = positions.iter().map(|(row, _)| *row).max().unwrap();
    let min_col = positions.iter().map(|(_, col)| *col).min().unwrap();
    let max_col = positions.iter().map(|(_, col)| *col).max().unwrap();

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if !positions.contains(&(row, col)) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
    println!("===============");
}

fn part_a(input: String) -> i32 {
    let mut positions = BTreeSet::new();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.as_bytes().iter().enumerate() {
            if *c == b'#' {
                positions.insert((row as i32, col as i32));
            }
        }
    }

    for i in 0..10 {
        print(&positions);

        let mut phase1 = BTreeMap::new();

        for (row, col) in positions.iter().cloned() {
            let mut found = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if positions.contains(&(row + dr, col + dc)) {
                        found += 1;
                    }
                }
            }

            if found <= 1 {
                *phase1.entry((row, col)).or_insert(0) += 1;
                continue;
            }

            let dirs = (0..4).cycle().skip(i).take(4);
            let mut dr = 0;
            let mut dc = 0;
            for dir in dirs {
                match dir {
                    0 => {
                        if !positions.contains(&(row - 1, col))
                            && !positions.contains(&(row - 1, col - 1))
                            && !positions.contains(&(row - 1, col + 1))
                        {
                            dr = -1;
                            break;
                        }
                    }
                    1 => {
                        if !positions.contains(&(row + 1, col))
                            && !positions.contains(&(row + 1, col - 1))
                            && !positions.contains(&(row + 1, col + 1))
                        {
                            dr = 1;
                            break;
                        }
                    }
                    2 => {
                        if !positions.contains(&(row, col - 1))
                            && !positions.contains(&(row - 1, col - 1))
                            && !positions.contains(&(row + 1, col - 1))
                        {
                            dc = -1;
                            break;
                        }
                    }
                    3 => {
                        if !positions.contains(&(row, col + 1))
                            && !positions.contains(&(row - 1, col + 1))
                            && !positions.contains(&(row + 1, col + 1))
                        {
                            dc = 1;
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            }

            *phase1.entry((row + dr, col + dc)).or_insert(0) += 1;
        }

        let mut phase2 = BTreeSet::new();

        for (row, col) in positions.iter().cloned() {
            let mut found = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if positions.contains(&(row + dr, col + dc)) {
                        found += 1;
                    }
                }
            }

            if found <= 1 {
                phase2.insert((row, col));
                continue;
            }

            let dirs = (0..4).cycle().skip(i).take(4);
            let mut dr = 0;
            let mut dc = 0;
            for dir in dirs {
                match dir {
                    0 => {
                        if !positions.contains(&(row - 1, col))
                            && !positions.contains(&(row - 1, col - 1))
                            && !positions.contains(&(row - 1, col + 1))
                        {
                            dr = -1;
                            break;
                        }
                    }
                    1 => {
                        if !positions.contains(&(row + 1, col))
                            && !positions.contains(&(row + 1, col - 1))
                            && !positions.contains(&(row + 1, col + 1))
                        {
                            dr = 1;
                            break;
                        }
                    }
                    2 => {
                        if !positions.contains(&(row, col - 1))
                            && !positions.contains(&(row - 1, col - 1))
                            && !positions.contains(&(row + 1, col - 1))
                        {
                            dc = -1;
                            break;
                        }
                    }
                    3 => {
                        if !positions.contains(&(row, col + 1))
                            && !positions.contains(&(row - 1, col + 1))
                            && !positions.contains(&(row + 1, col + 1))
                        {
                            dc = 1;
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            }

            if *phase1.get(&(row + dr, col + dc)).unwrap_or(&0) <= 1 {
                phase2.insert((row + dr, col + dc));
            } else {
                phase2.insert((row, col));
            }
        }

        positions = phase2;
    }

    let min_row = positions.iter().map(|(row, _)| *row).min().unwrap();
    let max_row = positions.iter().map(|(row, _)| *row).max().unwrap();
    let min_col = positions.iter().map(|(_, col)| *col).min().unwrap();
    let max_col = positions.iter().map(|(_, col)| *col).max().unwrap();

    let mut count = 0;
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if !positions.contains(&(row, col)) {
                count += 1;
            }
        }
    }

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if !positions.contains(&(row, col)) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
    println!("===============");

    count
}

fn part_b(input: String) -> i32 {
    let mut positions = BTreeSet::new();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.as_bytes().iter().enumerate() {
            if *c == b'#' {
                positions.insert((row as i32, col as i32));
            }
        }
    }

    for i in 0..1000000 {
        print(&positions);

        let mut phase1 = BTreeMap::new();

        for (row, col) in positions.iter().cloned() {
            let mut found = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if positions.contains(&(row + dr, col + dc)) {
                        found += 1;
                    }
                }
            }

            if found <= 1 {
                *phase1.entry((row, col)).or_insert(0) += 1;
                continue;
            }

            let dirs = (0..4).cycle().skip(i).take(4);
            let mut dr = 0;
            let mut dc = 0;
            for dir in dirs {
                match dir {
                    0 => {
                        if !positions.contains(&(row - 1, col))
                            && !positions.contains(&(row - 1, col - 1))
                            && !positions.contains(&(row - 1, col + 1))
                        {
                            dr = -1;
                            break;
                        }
                    }
                    1 => {
                        if !positions.contains(&(row + 1, col))
                            && !positions.contains(&(row + 1, col - 1))
                            && !positions.contains(&(row + 1, col + 1))
                        {
                            dr = 1;
                            break;
                        }
                    }
                    2 => {
                        if !positions.contains(&(row, col - 1))
                            && !positions.contains(&(row - 1, col - 1))
                            && !positions.contains(&(row + 1, col - 1))
                        {
                            dc = -1;
                            break;
                        }
                    }
                    3 => {
                        if !positions.contains(&(row, col + 1))
                            && !positions.contains(&(row - 1, col + 1))
                            && !positions.contains(&(row + 1, col + 1))
                        {
                            dc = 1;
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            }

            *phase1.entry((row + dr, col + dc)).or_insert(0) += 1;
        }

        let mut phase2 = BTreeSet::new();

        for (row, col) in positions.iter().cloned() {
            let mut found = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if positions.contains(&(row + dr, col + dc)) {
                        found += 1;
                    }
                }
            }

            if found <= 1 {
                phase2.insert((row, col));
                continue;
            }

            let dirs = (0..4).cycle().skip(i).take(4);
            let mut dr = 0;
            let mut dc = 0;
            for dir in dirs {
                match dir {
                    0 => {
                        if !positions.contains(&(row - 1, col))
                            && !positions.contains(&(row - 1, col - 1))
                            && !positions.contains(&(row - 1, col + 1))
                        {
                            dr = -1;
                            break;
                        }
                    }
                    1 => {
                        if !positions.contains(&(row + 1, col))
                            && !positions.contains(&(row + 1, col - 1))
                            && !positions.contains(&(row + 1, col + 1))
                        {
                            dr = 1;
                            break;
                        }
                    }
                    2 => {
                        if !positions.contains(&(row, col - 1))
                            && !positions.contains(&(row - 1, col - 1))
                            && !positions.contains(&(row + 1, col - 1))
                        {
                            dc = -1;
                            break;
                        }
                    }
                    3 => {
                        if !positions.contains(&(row, col + 1))
                            && !positions.contains(&(row - 1, col + 1))
                            && !positions.contains(&(row + 1, col + 1))
                        {
                            dc = 1;
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            }

            if *phase1.get(&(row + dr, col + dc)).unwrap_or(&0) <= 1 {
                phase2.insert((row + dr, col + dc));
            } else {
                phase2.insert((row, col));
            }
        }

        if phase2 == positions {
            return i as i32 + 1;
        }

        positions = phase2;
    }

    let min_row = positions.iter().map(|(row, _)| *row).min().unwrap();
    let max_row = positions.iter().map(|(row, _)| *row).max().unwrap();
    let min_col = positions.iter().map(|(_, col)| *col).min().unwrap();
    let max_col = positions.iter().map(|(_, col)| *col).max().unwrap();

    let mut count = 0;
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if !positions.contains(&(row, col)) {
                count += 1;
            }
        }
    }

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if !positions.contains(&(row, col)) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
    println!("===============");

    count
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
