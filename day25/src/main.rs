use clap::Parser;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn part_a(input: String) -> String {
    let mut result: i64 = 0;

    for line in input.lines() {
        let line_s = line;
        let line = line.as_bytes();

        let power = line.len() as i64 - 1;
        let mut acc = 1;
        for _ in 0..power {
            acc *= 5;
        }

        let mut a = 0;
        for byte in line {
            let v = match *byte {
                b'0' => 0,
                b'1' => 1,
                b'2' => 2,
                b'=' => -2,
                b'-' => -1,
                _ => unreachable!(),
            };
            a += acc * v;
            acc /= 5;
        }

        println!("line: {} => {}", line_s, a);
        result += a;
    }

    let mut s = String::new();

    while result > 0 {
        let m = result % 5;
        let c = match m {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                result += 5;
                '='
            }
            4 => {
                result += 5;
                '-'
            }
            _ => unreachable!(),
        };

        s.insert(0, c);

        result /= 5;
    }

    s
}

fn part_b(_input: String) -> i64 {
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
