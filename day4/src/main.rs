use clap::Parser;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn part_a(input: String) -> i32 {
    input
        .lines()
        .map(|line| {
            if let [a, b, c, d] = line
                .split(&[',', ';'])
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()[..]
            {
                ((a >= c && b <= d) || (c >= a && d <= b)) as i32
            } else {
                0
            }
        })
        .sum()
}

fn part_b(input: String) -> i32 {
    let mut s = 0;
    for line in input.lines() {
        let x = line.split(',').collect::<Vec<_>>();
        let l = x[0];
        let r = x[1];

        let e = l.split('-').collect::<Vec<_>>();
        let f = r.split('-').collect::<Vec<_>>();

        let a: i32 = e[0].parse().unwrap();
        let b: i32 = e[1].parse().unwrap();
        let c: i32 = f[0].parse().unwrap();
        let d: i32 = f[1].parse().unwrap();

        s += ((c >= a && c <= b) || (d >= a && d <= b) || (a >= c && a <= d) || (b >= c && b <= d))
            as i32;
    }
    s
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
