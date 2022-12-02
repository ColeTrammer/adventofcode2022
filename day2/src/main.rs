use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone)]
enum Outcome {
    Lose = 0,
    Tie = 3,
    Win = 6,
}

fn score(opp: Move, us: Move) -> i32 {
    match (opp, us) {
        (Move::Rock, Move::Scissors) => 0,
        (Move::Paper, Move::Rock) => 0,
        (Move::Scissors, Move::Paper) => 0,
        (Move::Paper, Move::Scissors) => 6,
        (Move::Scissors, Move::Rock) => 6,
        (Move::Rock, Move::Paper) => 6,
        _ => 3,
    }
}

fn choose(opp: Move, out: Outcome) -> Move {
    match (opp, out) {
        (Move::Rock, Outcome::Win) => Move::Paper,
        (Move::Rock, Outcome::Tie) => Move::Rock,
        (Move::Rock, Outcome::Lose) => Move::Scissors,
        (Move::Paper, Outcome::Win) => Move::Scissors,
        (Move::Paper, Outcome::Tie) => Move::Paper,
        (Move::Paper, Outcome::Lose) => Move::Rock,
        (Move::Scissors, Outcome::Win) => Move::Rock,
        (Move::Scissors, Outcome::Tie) => Move::Scissors,
        (Move::Scissors, Outcome::Lose) => Move::Paper,
    }
}

fn part_a(guide: &Vec<(Move, Move, Outcome)>) -> i32 {
    guide
        .iter()
        .map(|(opp, us, _)| (*us as i32) + score(*opp, *us))
        .sum()
}

fn part_b(guide: &Vec<(Move, Move, Outcome)>) -> i32 {
    guide
        .iter()
        .map(|(opp, _, outcome)| {
            let us = choose(*opp, *outcome);
            (us as i32) + (*outcome as i32)
        })
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
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input = File::open(args.input.unwrap_or("input.txt".into()))?;

    let mut guide = vec![];

    for line in BufReader::new(input).lines() {
        let line = line?;
        let split = line.split(" ").collect::<Vec<_>>();
        let opp = split[0];
        let us = split[1];

        let opp_move = match opp.as_bytes()[0] {
            b'A' => Move::Rock,
            b'B' => Move::Paper,
            b'C' => Move::Scissors,
            _ => unreachable!(),
        };

        let our_move = match us.as_bytes()[0] {
            b'X' => Move::Rock,
            b'Y' => Move::Paper,
            b'Z' => Move::Scissors,
            _ => unreachable!(),
        };

        let outcome = match us.as_bytes()[0] {
            b'X' => Outcome::Lose,
            b'Y' => Outcome::Tie,
            b'Z' => Outcome::Win,
            _ => unreachable!(),
        };

        guide.push((opp_move, our_move, outcome));
    }

    if args.part_b {
        println!("Part B: {:?}", part_b(&guide));
    } else {
        println!("Part A: {:?}", part_a(&guide));
    }

    Ok(())
}
