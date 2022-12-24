#![feature(int_roundings)]

use clap::Parser;
use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn eval(
    time: i32,
    res: (i32, i32, i32, i32),
    income: (i32, i32, i32, i32),
    blueprint: (i32, i32, (i32, i32), (i32, i32)),
    r: &mut i32,
    cache: &mut HashMap<(i32, (i32, i32, i32, i32), (i32, i32, i32, i32)), i32>,
) -> i32 {
    if time < 0 {
        return 0;
    } else if time == 0 {
        *r = res.3;
        return res.3;
    } else if let Some(result) = cache.get(&(time, res, income)) {
        return *result;
    }

    let (ore, clay, (obs_ore, obs_clay), (geode_ore, geode_obs)) = blueprint;

    let mut best = res.3 + income.3 * time;

    // Even if we kept making new geode robots every turn, we
    // still wouldn't surpass the highest value seen so far, so
    // just bail.
    if best + (time - 1) * time / 2 <= *r {
        return 0;
    }

    fn time_until(current: i32, income: i32, max_t: i32, needed: i32) -> i32 {
        if current >= needed {
            return 0;
        }
        if income == 0 {
            return max_t;
        }
        // current + income * t >= needed
        // income * t >= needed - current
        // t >= (needed - current) / income
        // t = ceil((needed - current) / income)
        (needed - current).div_ceil(income)
    }

    // Try making ore.
    {
        let time_wait = time_until(res.0, income.0, time, ore);
        let new_res = (
            res.0 + income.0 * (time_wait + 1) - ore,
            res.1 + income.1 * (time_wait + 1),
            res.2 + income.2 * (time_wait + 1),
            res.3 + income.3 * (time_wait + 1),
        );
        let new_income = (income.0 + 1, income.1, income.2, income.3);
        best = best.max(eval(
            time - time_wait - 1,
            new_res,
            new_income,
            blueprint,
            r,
            cache,
        ));
    }

    // Try making clay.
    {
        let time_wait = time_until(res.0, income.0, time, clay);
        let new_res = (
            res.0 + income.0 * (time_wait + 1) - clay,
            res.1 + income.1 * (time_wait + 1),
            res.2 + income.2 * (time_wait + 1),
            res.3 + income.3 * (time_wait + 1),
        );
        let new_income = (income.0, income.1 + 1, income.2, income.3);
        best = best.max(eval(
            time - time_wait - 1,
            new_res,
            new_income,
            blueprint,
            r,
            cache,
        ));
    }

    // Try making obs.
    {
        let time_wait = time_until(res.0, income.0, time, obs_ore)
            .max(time_until(res.1, income.1, time, obs_clay));
        let new_res = (
            res.0 + income.0 * (time_wait + 1) - obs_ore,
            res.1 + income.1 * (time_wait + 1) - obs_clay,
            res.2 + income.2 * (time_wait + 1),
            res.3 + income.3 * (time_wait + 1),
        );
        let new_income = (income.0, income.1, income.2 + 1, income.3);
        best = best.max(eval(
            time - time_wait - 1,
            new_res,
            new_income,
            blueprint,
            r,
            cache,
        ));
    }

    // Try making geo.
    {
        let time_wait = time_until(res.0, income.0, time, geode_ore)
            .max(time_until(res.2, income.2, time, geode_obs));
        let new_res = (
            res.0 + income.0 * (time_wait + 1) - geode_ore,
            res.1 + income.1 * (time_wait + 1),
            res.2 + income.2 * (time_wait + 1) - geode_obs,
            res.3 + income.3 * (time_wait + 1),
        );
        let new_income = (income.0, income.1, income.2, income.3 + 1);
        best = best.max(eval(
            time - time_wait - 1,
            new_res,
            new_income,
            blueprint,
            r,
            cache,
        ));
    }

    cache.insert((time, res, income), best);
    best
}

fn part_a(input: String) -> i32 {
    let mut blueprints = vec![];
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();

        for (a, b, c, d, e, f) in parts
            .iter()
            .filter(|x| x.parse::<i32>().is_ok())
            .map(|x| x.parse::<i32>().unwrap())
            .tuple_windows()
        {
            blueprints.push((a, b, (c, d), (e, f)));
        }
    }

    let mut result = 0;

    for (ind, blueprint) in blueprints.iter().enumerate() {
        let mut cache = HashMap::new();

        let mut r = 0;
        let score = eval(
            24,
            (0, 0, 0, 0),
            (1, 0, 0, 0),
            *blueprint,
            &mut r,
            &mut cache,
        );

        println!("{}: {}", ind + 1, score);

        result += score * ((ind + 1) as i32);
    }

    result
}

fn part_b(input: String) -> i64 {
    let mut blueprints = vec![];
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();

        for (a, b, c, d, e, f) in parts
            .iter()
            .filter(|x| x.parse::<i32>().is_ok())
            .map(|x| x.parse::<i32>().unwrap())
            .tuple_windows()
        {
            blueprints.push((a, b, (c, d), (e, f)));
        }
    }

    let mut result = 1;

    for (ind, blueprint) in blueprints.iter().take(3).enumerate() {
        let mut cache = HashMap::new();

        let mut r = 0;
        let score = eval(
            32,
            (0, 0, 0, 0),
            (1, 0, 0, 0),
            *blueprint,
            &mut r,
            &mut cache,
        );

        println!("{}: {}", ind + 1, score);

        result *= score as i64;
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
        println!("Part B: {:?}", part_b(input.clone()));
    } else {
        println!("Part A: {:?}", part_a(input.clone()));
    }

    Ok(())
}
