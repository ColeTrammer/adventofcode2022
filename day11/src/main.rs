use clap::Parser;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn part_a(input: String) -> i64 {
    let monkies = input.split("\n\n").collect::<Vec<_>>();
    let count = monkies.len();

    let mut items = vec![vec![]; count];
    let mut counts = vec![0; count];

    enum Op {
        Add(i64),
        Mul(i64),
        MulSelf(),
    }

    struct Monkey {
        test: i64,
        yes: i64,
        no: i64,
        op: Op,
    }

    let mut mons = vec![];

    for (i, monkey) in monkies.into_iter().enumerate() {
        let parts = monkey.lines().collect::<Vec<_>>();
        if let Some(suffix) = parts[1].strip_prefix("  Starting items: ") {
            for n in suffix.split(", ").map(|x| x.parse::<i64>().unwrap()) {
                items[i].push(n);
            }
        }

        let op = parts[2]
            .strip_prefix("  Operation: new = old ")
            .map(|suffix| {
                if suffix == "* old" {
                    Op::MulSelf()
                } else if suffix.starts_with("+") {
                    Op::Add(suffix.strip_prefix("+ ").unwrap().parse().unwrap())
                } else {
                    Op::Mul(suffix.strip_prefix("* ").unwrap().parse().unwrap())
                }
            })
            .unwrap();

        let ns = parts[3..]
            .iter()
            .map(|x| x.split(" ").last().unwrap().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        mons.push(Monkey {
            test: ns[0],
            yes: ns[1],
            no: ns[2],
            op,
        });
    }

    for _ in 0..10000 {
        // println!("{:?}", items);
        for (index, monkey) in mons.iter().enumerate() {
            for value in items[index].clone() {
                counts[index] += 1;

                let new_value = match monkey.op {
                    Op::Add(v) => value + v,
                    Op::Mul(v) => value * v,
                    Op::MulSelf() => value * value,
                } / 3;

                if new_value % monkey.test == 0 {
                    items[monkey.yes as usize].push(new_value);
                } else {
                    items[monkey.no as usize].push(new_value);
                }
            }
            items[index].clear();
        }
    }

    counts.sort();
    println!("{:?}", counts);

    return counts[counts.len() - 1] * counts[counts.len() - 2];
}

fn part_b(input: String) -> i64 {
    let monkies = input.split("\n\n").collect::<Vec<_>>();
    let count = monkies.len();

    let mut items = vec![vec![]; count];
    let mut counts = vec![0; count];

    enum Op {
        Add(i64),
        Mul(i64),
        MulSelf(),
    }

    struct Monkey {
        test: i64,
        yes: i64,
        no: i64,
        op: Op,
    }

    let mut mons = vec![];

    for (i, monkey) in monkies.into_iter().enumerate() {
        let parts = monkey.lines().collect::<Vec<_>>();
        if let Some(suffix) = parts[1].strip_prefix("  Starting items: ") {
            for n in suffix.split(", ").map(|x| x.parse::<i64>().unwrap()) {
                items[i].push(n);
            }
        }

        let op = parts[2]
            .strip_prefix("  Operation: new = old ")
            .map(|suffix| {
                if suffix == "* old" {
                    Op::MulSelf()
                } else if suffix.starts_with("+") {
                    Op::Add(suffix.strip_prefix("+ ").unwrap().parse().unwrap())
                } else {
                    Op::Mul(suffix.strip_prefix("* ").unwrap().parse().unwrap())
                }
            })
            .unwrap();

        let ns = parts[3..]
            .iter()
            .map(|x| x.split(" ").last().unwrap().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        mons.push(Monkey {
            test: ns[0],
            yes: ns[1],
            no: ns[2],
            op,
        });
    }

    let modd: i64 = mons.iter().map(|mon| mon.test).product();

    for _ in 0..10000 {
        // println!("{:?}", items);
        for (index, monkey) in mons.iter().enumerate() {
            for value in items[index].clone() {
                counts[index] += 1;

                let new_value = match monkey.op {
                    Op::Add(v) => value + v,
                    Op::Mul(v) => value * v,
                    Op::MulSelf() => value * value,
                };

                if new_value % monkey.test == 0 {
                    items[monkey.yes as usize].push(new_value % modd);
                } else {
                    items[monkey.no as usize].push(new_value % modd);
                }
            }
            items[index].clear();
        }
    }

    counts.sort();
    println!("{:?}", counts);

    return counts[counts.len() - 1] * counts[counts.len() - 2];
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
