use clap::Parser;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn part_a(input: String) -> i32 {
    let mut xs = vec![];
    let mut x = 1 as i32;
    for line in input.lines() {
        if line.starts_with("noop") {
            xs.push(x);
        } else if let Some(suffix) = line.strip_prefix("addx ") {
            let v = suffix.parse::<i32>().unwrap();
            xs.push(x);
            xs.push(x);
            x += v;
        } else {
            println!("??");
        }
    }

    println!("{:?}", xs);

    xs.into_iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, value)| {
            println!("i: {}, v: {}", i, value);
            (i as i32 + 1) * value
        })
        .sum()
}

fn part_b(input: String) -> String {
    let mut x = 1 as i32;
    let mut cycle_count = 0;
    let mut result = "\n".to_owned();
    for line in input.lines() {
        let cycles = if line.starts_with("noop") { 1 } else { 2 };

        for _ in 0..cycles {
            let pixel: i32 = cycle_count % 40;
            if pixel.abs_diff(x) <= 1 {
                result.push('#');
            } else {
                result.push('.');
            }
            cycle_count += 1;

            if cycle_count % 40 == 0 {
                result.push('\n');
            }
        }

        if let Some(suffix) = line.strip_prefix("addx ") {
            let v = suffix.parse::<i32>().unwrap();
            x += v;
        }
    }

    result
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
        println!("Part B: {}", part_b(input.clone()));
    } else {
        println!("Part A: {:?}", part_a(input.clone()));
    }

    Ok(())
}
