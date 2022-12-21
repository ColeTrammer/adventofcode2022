use clap::Parser;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn part_a(input: String) -> i32 {
    let mut edges = HashMap::new();

    for line in input.lines() {
        for coords in line
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .tuple_windows()
        {
            edges.insert(coords, HashSet::new());
        }
    }

    for (x, y, z) in edges.keys().cloned().collect::<Vec<_>>() {
        for (dx, dy, dz) in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let (nx, ny, nz) = (x + dx, y + dy, z + dz);
            if edges.contains_key(&(nx, ny, nz)) {
                edges.get_mut(&(x, y, z)).unwrap().insert((nx, ny, nz));
            }
        }
    }

    let mut queue = edges.keys().cloned().collect::<VecDeque<_>>();

    let mut set = HashSet::new();

    let mut count = 0;
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        if set.contains(&node) {
            continue;
        }

        let mut q = VecDeque::new();
        q.push_back(node);

        let mut c = 0;
        let mut e = 0;
        while !q.is_empty() {
            let n = q.pop_front().unwrap();
            if set.contains(&n) {
                continue;
            }

            c += 1;

            set.insert(n);

            for ed in edges.get(&n).unwrap() {
                e += 1;
                q.push_back(*ed);
            }
        }

        count += c * 6 - e;
    }

    count
}

fn part_b(input: String) -> i32 {
    let mut nodes = HashSet::new();

    for line in input.lines() {
        for coords in line
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .tuple_windows()
        {
            nodes.insert(coords);
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back((100, 100, 100));

    let mut set = HashSet::new();

    while !queue.is_empty() {
        let (x, y, z) = queue.pop_front().unwrap();
        if set.contains(&(x, y, z)) {
            continue;
        }

        set.insert((x, y, z));

        for (dx, dy, dz) in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let (nx, ny, nz) = (x + dx, y + dy, z + dz);
            if !nodes.contains(&(nx, ny, nz))
                && nx >= -50
                && nx <= 100
                && ny >= -50
                && ny <= 100
                && nz >= -50
                && nz <= 100
            {
                queue.push_back((nx, ny, nz));
            }
        }
    }

    let mut count = 0;

    for (x, y, z) in nodes.iter().cloned().collect::<Vec<_>>() {
        for (dx, dy, dz) in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let (nx, ny, nz) = (x + dx, y + dy, z + dz);
            if !nodes.contains(&(nx, ny, nz)) && set.contains(&(nx, ny, nz)) {
                count += 1;
            }
        }
    }

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
