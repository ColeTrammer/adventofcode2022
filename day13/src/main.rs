use clap::Parser;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::vec;

#[derive(Clone, Debug)]
enum Value {
    One(i32),
    Many(Vec<Box<Value>>),
}

fn parse(s: &str) -> Value {
    let s = s.as_bytes();
    let mut i = 0;

    let mut stack = vec![];

    while i < s.len() {
        if s[i] == b'[' {
            stack.push(vec![]);
            i += 1;
        } else if s[i] == b']' {
            if stack.len() > 1 {
                let arr = stack.pop().unwrap();
                stack.last_mut().unwrap().push(Box::new(Value::Many(arr)));
            }
            i += 1;
        } else if s[i] == b' ' || s[i] == b',' {
            i += 1;
        } else {
            let to_skip = s[i..].iter().take_while(|p| p.is_ascii_digit()).count();
            let slice = &s[i..i + to_skip];
            let n = String::from_utf8(Vec::from(slice))
                .unwrap()
                .parse::<i32>()
                .unwrap();
            stack.last_mut().unwrap().push(Box::new(Value::One(n)));
            i += to_skip;
        }
    }

    Value::Many(stack.pop().unwrap())
}

fn in_order(a: Value, b: Value) -> std::cmp::Ordering {
    match (a, b) {
        (Value::One(x), Value::One(y)) => return x.cmp(&y),
        (Value::One(x), Value::Many(y)) => {
            return in_order(Value::Many(vec![Box::new(Value::One(x))]), Value::Many(y));
        }
        (Value::Many(x), Value::One(y)) => {
            return in_order(Value::Many(x), Value::Many(vec![Box::new(Value::One(y))]));
        }
        (Value::Many(x), Value::Many(y)) => {
            let mut i = 0;

            while i < x.len() && i < y.len() {
                let result = in_order(*x[i].clone(), *y[i].clone());
                if result != std::cmp::Ordering::Equal {
                    return result;
                }
                i += 1;
            }
            return x.len().cmp(&y.len());
        }
    }
}

fn part_a(input: String) -> i32 {
    let mut sum = 0;
    for (index, pairs) in input.split("\n\n").enumerate() {
        let x = pairs.lines().map(parse).collect::<Vec<_>>();
        let a = x[0].clone();
        let b = x[1].clone();

        if in_order(a, b) == std::cmp::Ordering::Less {
            sum += (index + 1) as i32;
        }
    }

    sum
}

fn part_b(input: String) -> i32 {
    let mut values = vec![];

    for pairs in input.split("\n\n") {
        let x = pairs.lines().map(parse).collect::<Vec<_>>();
        let a = x[0].clone();
        let b = x[1].clone();
        values.push(a);
        values.push(b);
    }

    values.push(Value::Many(vec![Box::new(Value::Many(vec![Box::new(
        Value::One(2),
    )]))]));
    values.push(Value::Many(vec![Box::new(Value::Many(vec![Box::new(
        Value::One(6),
    )]))]));

    values.sort_by(|a, b| in_order(a.clone(), b.clone()));

    values
        .into_iter()
        .enumerate()
        .filter(|(_, v)| match v {
            Value::Many(x) => {
                x.len() == 1
                    && match *x[0].clone() {
                        Value::Many(y) => {
                            y.len() == 1
                                && match *y[0] {
                                    Value::One(z) => z == 2 || z == 6,
                                    _ => false,
                                }
                        }
                        _ => false,
                    }
            }
            _ => false,
        })
        .map(|(i, _)| (i + 1) as i32)
        .product()
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
