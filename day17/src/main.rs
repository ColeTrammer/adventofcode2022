use clap::Parser;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn part_a(input: String, m: i32) -> i32 {
    let rocks: Vec<Vec<(i32, i32)>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    // true=left, false=right
    let dirs = input
        .as_bytes()
        .into_iter()
        .map(|x| *x == b'<')
        .collect::<Vec<_>>();

    let mut grid = vec![0 as u8; 4];

    let mut highest_point = 0;

    let mut rock_index = 0;
    let mut wind_index = 0;
    for _ in 0..m {
        let rock = &rocks[rock_index];
        rock_index += 1;
        rock_index %= rocks.len();

        let mut go_down = false;
        let (mut rx, mut ry) = (2, highest_point + 3);

        let max_y = rock.iter().map(|(_, y)| ry + *y).max().unwrap();
        while max_y >= grid.len() as i32 {
            grid.push(0);
        }

        let get = |x, y| -> bool {
            let row = y as usize;
            (grid[row] & (1 << (x as u8))) != 0
        };

        fn set(grid: &mut Vec<u8>, x: i32, y: i32) {
            let row = y as usize;
            grid[row] |= 1 << (x as u8)
        }

        loop {
            let (dx, dy) = if go_down {
                (0, -1)
            } else {
                let dx = if dirs[wind_index] { -1 } else { 1 };
                wind_index += 1;
                wind_index %= dirs.len();
                (dx, 0)
            };

            // println!("rx={rx} ry={ry}");

            rx += dx;
            ry += dy;

            let is_valid = rock.iter().all(|(x, y)| {
                let x = rx + *x;
                let y = ry + *y;
                if x < 0 || x >= 7 || y < 0 {
                    return false;
                }
                // let v = !get(x, y);
                // println!("g x={x} y={y} v={v}");
                !get(x, y)
            });
            if !is_valid {
                rx -= dx;
                ry -= dy;
            }
            if !is_valid && go_down {
                // println!("rx={rx} ry={ry}");
                rock.iter()
                    .inspect(|(x, y)| {
                        let x = rx + *x;
                        let y = ry + *y;
                        // println!("s x={x} y={y}");
                        set(&mut grid, x, y);
                    })
                    .all(|_| true);

                highest_point =
                    highest_point.max(rock.iter().map(|(_, y)| ry + *y).max().unwrap() + 1);
                break;
            }
            go_down = !go_down;
        }

        // for row in grid.iter().rev() {
        //     let mut out = String::new();
        //     for i in 0..7 {
        //         if *row & (1 << i) == 0 {
        //             out.push('.');
        //         } else {
        //             out.push('#');
        //         }
        //     }
        //     println!("{}", out);
        // }
        // println!("=========================");
    }

    highest_point
}

// By printing the grid every time the situation
// should cycle (when rock_index=0 and wind_index=0)
//     which is approximately ~(5 * 1000),
// I confirmed that the grid is the same after
// this many iterations of the simulation. The
// problem can then be solved by simulating on
// up until that point, and then multiplying the
// height gain by the number of the times the cycle
// occurs.

// Therefore, the solver assumes that a cycle will
// occur at this point for all inputs.

// For the actual problem, I solved it using
// print statements and a calculator:

// Every 1740 rocks: 2681 blocks are raised.
// 1000000000000 / 1740 =
// 574712643 rem 1180
// So, simply run until 1740 + 1180 rocks fall.
// 4480 + 2681 * (574712643 - 1) = 1540804597682

fn find_cycle_length(input: String) -> i64 {
    let rocks: Vec<Vec<(i64, i64)>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    // true=left, false=right
    let dirs = input
        .as_bytes()
        .into_iter()
        .map(|x| *x == b'<')
        .collect::<Vec<_>>();

    let mut grid = vec![0 as u8; 4];

    let mut highest_point = 0 as i64;

    let mut rock_index = 0;
    let mut wind_index = 0;
    let mut count_for_cycle = 0;
    let mut last_cycle = 0;
    let m = 1000000000000 as i64;
    // let m = 1740 + 1180;
    for iter in 0..m {
        let rock = &rocks[rock_index];
        rock_index += 1;
        rock_index %= rocks.len();

        let mut go_down = false;
        let (mut rx, mut ry) = (2, highest_point + 3);

        let max_y = rock.iter().map(|(_, y)| ry + *y).max().unwrap();
        while max_y >= grid.len() as i64 {
            grid.push(0);
        }

        let get = |x, y| -> bool {
            let row = y as usize;
            (grid[row] & (1 << (x as u8))) != 0
        };

        fn set(grid: &mut Vec<u8>, x: i64, y: i64) {
            let row = y as usize;
            grid[row] |= 1 << (x as u8)
        }

        loop {
            if wind_index == 0 && rock_index == 0 {
                if !go_down {
                    for row in grid.iter().rev().take(20) {
                        let mut out = String::new();
                        for i in 0..7 {
                            if *row & (1 << i) == 0 {
                                out.push('.');
                            } else {
                                out.push('#');
                            }
                        }
                        println!("{}", out);
                    }
                    println!("=========================");
                    println!("{} => {}", iter, highest_point);
                    println!("=========================");
                }

                // count_for_cycle += 1;

                // if count_for_cycle >= 500 {
                //     return iter - last_cycle;
                // }
                // if !go_down {
                //     last_cycle = iter;
                // }
            }

            let (dx, dy) = if go_down {
                (0, -1)
            } else {
                let dx = if dirs[wind_index] { -1 } else { 1 };
                wind_index += 1;
                wind_index %= dirs.len();
                (dx, 0)
            };

            // println!("rx={rx} ry={ry}");

            rx += dx;
            ry += dy;

            let is_valid = rock.iter().all(|(x, y)| {
                let x = rx + *x;
                let y = ry + *y;
                if x < 0 || x >= 7 || y < 0 {
                    return false;
                }
                // let v = !get(x, y);
                // println!("g x={x} y={y} v={v}");
                !get(x, y)
            });
            if !is_valid {
                rx -= dx;
                ry -= dy;
            }
            if !is_valid && go_down {
                // println!("rx={rx} ry={ry}");
                rock.iter()
                    .inspect(|(x, y)| {
                        let x = rx + *x;
                        let y = ry + *y;
                        // println!("s x={x} y={y}");
                        set(&mut grid, x, y);
                    })
                    .all(|_| true);

                highest_point =
                    highest_point.max(rock.iter().map(|(_, y)| ry + *y).max().unwrap() + 1);
                break;
            }
            go_down = !go_down;
        }
    }

    0
}

fn part_b(input: String) -> i64 {
    let cycle_length = find_cycle_length(input.clone());

    let m = 1000000000000 as i64;

    let per_cycle = part_a(input.clone(), (cycle_length + 1) as i32) as i64;
    let cycle_with_rem = part_a(input, (cycle_length + m % cycle_length) as i32) as i64;

    let cycles = m / cycle_length;

    println!("cycle_length={cycle_length} cycles={cycles} per_cycle={per_cycle} cycle_with_rem={cycle_with_rem}");

    per_cycle * (cycles - 1) + cycle_with_rem
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
        println!("Part A: {:?}", part_a(input.clone(), 2022));
    }

    Ok(())
}
