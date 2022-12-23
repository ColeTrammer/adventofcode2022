use clap::Parser;
use itertools::Itertools;
use std::collections::{BTreeMap, BinaryHeap, HashSet};
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

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
        fn eval(
            time: i32,
            res: (i32, i32, i32, i32),
            income: (i32, i32, i32, i32),
            blueprint: (i32, i32, (i32, i32), (i32, i32)),
            r: i32,
            cache: &mut BTreeMap<(i32, (i32, i32, i32, i32), (i32, i32, i32, i32), i32), i32>,
        ) -> i32 {
            if time <= 0 {
                return res.3;
            }

            if let Some(result) = cache.get(&(time, res, income, r)) {
                return *result;
            }

            let (ore, clay, (obs_ore, obs_clay), (geode_ore, geode_obs)) = blueprint;

            let new_res = (
                res.0 + income.0,
                res.1 + income.1,
                res.2 + income.2,
                res.3 + income.3,
            );

            let mut best = eval(time - 1, new_res, income, blueprint, r, cache);

            // Can build ore
            if res.0 >= ore && r <= 4 {
                let mut new_income = income;
                new_income.0 += 1;

                let mut new_res = new_res;
                new_res.0 -= ore;

                best = best.max(eval(time - 1, new_res, new_income, blueprint, 4, cache));
            }

            // Can build clay
            if res.0 >= clay && r <= 4 {
                let mut new_income = income;
                new_income.1 += 1;

                let mut new_res = new_res;
                new_res.0 -= clay;
                best = best.max(eval(time - 1, new_res, new_income, blueprint, 4, cache));
            }

            // Can build obs
            if res.0 >= obs_ore && res.1 >= obs_clay && r <= 4 {
                let mut new_income = income;
                new_income.2 += 1;

                let mut new_res = new_res;
                new_res.0 -= obs_ore;
                new_res.1 -= obs_clay;
                best = best.max(eval(time - 1, new_res, new_income, blueprint, 4, cache));
            }

            // Can build geo
            if res.0 >= geode_ore && res.2 >= geode_obs && r <= 4 {
                let mut new_income = income;
                new_income.3 += 1;

                let mut new_res = new_res;
                new_res.0 -= geode_ore;
                new_res.2 -= geode_obs;
                best = best.max(eval(time - 1, new_res, new_income, blueprint, 4, cache));
            }

            cache.insert((time, res, income, r), best);
            return best;
        }

        let mut cache = BTreeMap::new();

        let score = eval(24, (0, 0, 0, 0), (1, 0, 0, 0), *blueprint, 4, &mut cache);

        println!("{}: {}", ind + 1, score);

        result += score * ((ind + 1) as i32);
    }

    result
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, Hash)]
struct Node {
    time: i32,
    res: (i32, i32, i32, i32),
    income: (i32, i32, i32, i32),
}

impl Node {
    fn score(self) -> i128 {
        // let a = self.income.3 * self.time + self.res.3;
        // let b = self.income.2 * self.time + self.res.2;
        // let c = self.income.1 * self.time + self.res.1;
        // let d = self.income.0 * self.time + self.res.0;

        // let a = a as i128;
        // let b = b as i128;
        // let c = c as i128;
        // let d = d as i128;

        // -((a << 96) | (b << 64) | (c << 32) | d)
        -self.time as i128
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score().partial_cmp(&other.score())
    }
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
        // let mut queue = BinaryHeap::new();
        // let mut score = 0;

        // let mut visited = HashSet::new();

        // queue.push(Node {
        //     time: 32,
        //     res: (0, 0, 0, 0),
        //     income: (1, 0, 0, 0),
        // });
        // visited.insert(Node {
        //     time: 32,
        //     res: (0, 0, 0, 0),
        //     income: (1, 0, 0, 0),
        // });

        // while !queue.is_empty() {
        //     let Node { time, res, income } = queue.pop().unwrap();

        //     // println!("time: {time}, res: {res:?}, income: {income:?}");

        //     if time == 0 {
        //         // score = res.3;
        //         // break;
        //         let old_score = score;
        //         score = score.max(res.3);
        //         if score > old_score {
        //             println!("got new score: {}", score);
        //             continue;
        //         } else if score == old_score {
        //             continue;
        //         } else {
        //             println!("got old score {} - {}", score, res.3);
        //             continue;
        //         }
        //     }

        //     let (ore, clay, (obs_ore, obs_clay), (geode_ore, geode_obs)) = blueprint.clone();

        //     let new_res = (
        //         res.0 + income.0,
        //         res.1 + income.1,
        //         res.2 + income.2,
        //         res.3 + income.3,
        //     );

        //     if !visited.contains(&Node {
        //         time: time - 1,
        //         res: new_res,
        //         income,
        //     }) {
        //         queue.push(Node {
        //             time: time - 1,
        //             res: new_res,
        //             income,
        //         });
        //         visited.insert(Node {
        //             time: time - 1,
        //             res: new_res,
        //             income,
        //         });
        //     }

        //     // Can build ore
        //     if res.0 >= ore {
        //         let mut new_income = income;
        //         new_income.0 += 1;

        //         let mut new_res = new_res;
        //         new_res.0 -= ore;

        //         if !visited.contains(&Node {
        //             time: time - 1,
        //             res: new_res,
        //             income: new_income,
        //         }) {
        //             queue.push(Node {
        //                 time: time - 1,
        //                 res: new_res,
        //                 income: new_income,
        //             });
        //             visited.insert(Node {
        //                 time: time - 1,
        //                 res: new_res,
        //                 income: new_income,
        //             });
        //         }
        //     }

        //     // Can build clay
        //     if res.0 >= clay {
        //         let mut new_income = income;
        //         new_income.1 += 1;

        //         let mut new_res = new_res;
        //         new_res.0 -= clay;

        //         if !visited.contains(&Node {
        //             time: time - 1,
        //             res: new_res,
        //             income: new_income,
        //         }) {
        //             queue.push(Node {
        //                 time: time - 1,
        //                 res: new_res,
        //                 income: new_income,
        //             });
        //             visited.insert(Node {
        //                 time: time - 1,
        //                 res: new_res,
        //                 income: new_income,
        //             });
        //         }
        //     }

        //     // Can build obs
        //     if res.0 >= obs_ore && res.1 >= obs_clay {
        //         let mut new_income = income;
        //         new_income.2 += 1;

        //         let mut new_res = new_res;
        //         new_res.0 -= obs_ore;
        //         new_res.1 -= obs_clay;

        //         if !visited.contains(&Node {
        //             time: time - 1,
        //             res: new_res,
        //             income: new_income,
        //         }) {
        //             queue.push(Node {
        //                 time: time - 1,
        //                 res: new_res,
        //                 income: new_income,
        //             });
        //             visited.insert(Node {
        //                 time: time - 1,
        //                 res: new_res,
        //                 income: new_income,
        //             });
        //         }
        //     }

        //     // Can build geo
        //     if res.0 >= geode_ore && res.2 >= geode_obs {
        //         let mut new_income = income;
        //         new_income.3 += 1;

        //         let mut new_res = new_res;
        //         new_res.0 -= geode_ore;
        //         new_res.2 -= geode_obs;

        //         if !visited.contains(&Node {
        //             time: time - 1,
        //             res: new_res,
        //             income: new_income,
        //         }) {
        //             queue.push(Node {
        //                 time: time - 1,
        //                 res: new_res,
        //                 income: new_income,
        //             });
        //             visited.insert(Node {
        //                 time: time - 1,
        //                 res: new_res,
        //                 income: new_income,
        //             });
        //         }
        //     }
        // }

        fn eval(
            time: i32,
            res: (i32, i32, i32, i32),
            income: (i32, i32, i32, i32),
            blueprint: (i32, i32, (i32, i32), (i32, i32)),
            r: i32,
            cache: &mut BTreeMap<(i32, (i32, i32, i32, i32), (i32, i32, i32, i32), i32), i32>,
        ) -> i32 {
            if time <= 0 {
                return res.3;
            }

            // if let Some(result) = cache.get(&(time, res, income, r)) {
            // return *result;
            // }

            let (ore, clay, (obs_ore, obs_clay), (geode_ore, geode_obs)) = blueprint;

            let new_res = (
                res.0 + income.0,
                res.1 + income.1,
                res.2 + income.2,
                res.3 + income.3,
            );

            let mut best = eval(time - 1, new_res, income, blueprint, r, cache);

            // Can build ore
            if res.0 >= ore && r <= 4 {
                let mut new_income = income;
                new_income.0 += 1;

                let mut new_res = new_res;
                new_res.0 -= ore;

                best = best.max(eval(time - 1, new_res, new_income, blueprint, 4, cache));
            }

            // Can build clay
            if res.0 >= clay && r <= 4 {
                let mut new_income = income;
                new_income.1 += 1;

                let mut new_res = new_res;
                new_res.0 -= clay;
                best = best.max(eval(time - 1, new_res, new_income, blueprint, 4, cache));
            }

            // Can build obs
            if res.0 >= obs_ore && res.1 >= obs_clay && r <= 4 {
                let mut new_income = income;
                new_income.2 += 1;

                let mut new_res = new_res;
                new_res.0 -= obs_ore;
                new_res.1 -= obs_clay;
                best = best.max(eval(time - 1, new_res, new_income, blueprint, 4, cache));
            }

            // Can build geo
            if res.0 >= geode_ore && res.2 >= geode_obs && r <= 4 {
                let mut new_income = income;
                new_income.3 += 1;

                let mut new_res = new_res;
                new_res.0 -= geode_ore;
                new_res.2 -= geode_obs;
                best = best.max(eval(time - 1, new_res, new_income, blueprint, 4, cache));
            }

            // cache.insert((time, res, income, r), best);
            return best;
        }

        let mut cache = BTreeMap::new();

        let score = eval(32, (0, 0, 0, 0), (1, 0, 0, 0), *blueprint, 4, &mut cache);

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
