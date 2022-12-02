#![feature(binary_heap_drain_sorted)]

use std::collections::BinaryHeap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::vec::Vec;

fn part1(input: &Vec<Vec<i32>>) -> i32 {
    let values = input.iter().map(|x| x.iter().sum::<i32>());
    values.max().unwrap()
}

fn part2(input: &Vec<Vec<i32>>) -> i32 {
    input
        .iter()
        .map(|x| x.iter().sum::<i32>())
        .collect::<BinaryHeap<_>>()
        .drain_sorted()
        .take(3)
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Must specify path!");
        exit(1);
    }

    let file = File::open(&args[1]).unwrap();

    let mut input = vec![];
    let mut running = vec![];
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        if line.is_empty() {
            input.push(running.clone());
            running.clear();
        } else {
            running.push(line.parse::<i32>().unwrap());
        }
    }
    if !running.is_empty() {
        input.push(running.clone());
    }

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
