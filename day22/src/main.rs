use clap::Parser;
use itertools::Itertools;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn part_a(input: String) -> i64 {
    let mut grid = vec![];
    let mut dirs = vec![];

    for (g, d) in input.split("\n\n").tuple_windows() {
        let rows = g.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();

        let row_count = rows.len();
        let col_count = rows[0].len();

        for r in 0..row_count {
            let mut acc = vec![];
            for c in 0..col_count {
                acc.push(match rows[r].get(c) {
                    Some(b'.') => 1,
                    Some(b'#') => 2,
                    _ => 0,
                });
            }
            grid.push(acc);
        }

        let dd = d
            .split(|c: char| c.is_ascii_alphabetic())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut dr = d
            .split(|c: char| c.is_ascii_digit())
            .filter(|x| !x.is_empty())
            .map(|x| x.as_bytes()[0])
            .collect::<Vec<_>>();

        dr.push(b'S');

        dirs = dd.into_iter().zip(dr.into_iter()).collect();
    }

    let mut row = 0 as i32;
    let mut col = 0;
    let mut dir = 0;
    for (i, v) in grid[0].iter().enumerate() {
        if v == &1 {
            col = i as i32;
            break;
        }
    }

    // println!("sz: {} {}", grid.len(), grid[0].len());

    // println!("{row} {col} {dir}");
    for (step, d) in dirs {
        println!(
            ":: {} {}",
            match dir {
                0 => '>',
                1 => 'v',
                2 => '<',
                3 => '^',
                _ => '!',
            },
            d as char
        );

        assert!(
            grid.get((row) as usize)
                .and_then(|v| v.get((col) as usize).map(|x| *x))
                .unwrap()
                == 1
        );

        for _ in 0..step {
            let (mut dr, mut dc) = [(0, 1), (1, 0), (0, -1), (-1, 0)][dir as usize];

            let mut v = grid
                .get((row + dr) as usize)
                .and_then(|v| v.get((col + dc) as usize).map(|x| *x));

            let check = |opt: Option<i32>| !opt.is_some() || opt.unwrap() == 0;

            if check(v) {
                dr = -dr;
                dc = -dc;

                let mut nr = row;
                let mut nc = col;

                let mut last_good_row = nr;
                let mut last_good_col = nc;

                while grid
                    .get((nr + dr) as usize)
                    .and_then(|v| v.get((nc + dc) as usize).map(|x| *x))
                    .is_some()
                {
                    if grid
                        .get((nr + dr) as usize)
                        .and_then(|v| v.get((nc + dc) as usize).map(|x| *x))
                        .unwrap()
                        != 0
                    {
                        last_good_row = nr + dr;
                        last_good_col = nc + dc;
                    }
                    nr += dr;
                    nc += dc;
                }
                v = grid
                    .get((last_good_row) as usize)
                    .and_then(|v| v.get((last_good_col) as usize).map(|x| *x));

                dr = last_good_row - row;
                dc = last_good_col - col;

                assert!(row + dr == last_good_row);
                assert!(v.is_some());
            } else {
                assert!(v.is_some());
            }

            if v.unwrap() == 2 {
                break;
            }

            row += dr;
            col += dc;
            // println!("{row} {col} {dir}");

            assert!(
                grid.get((row) as usize)
                    .and_then(|v| v.get((col) as usize).map(|x| *x))
                    .unwrap()
                    == 1
            );
        }

        dir += if d == b'L' {
            -1
        } else if d == b'R' {
            1
        } else {
            0
        };
        if dir < 0 {
            dir += 4;
        }
        dir %= 4;
    }

    println!("{row} {col} {dir}");

    1000 * (row as i64 + 1) + 4 * (col as i64 + 1) + dir
}

fn part_b(input: String) -> i64 {
    let mut grid = vec![];
    let mut dirs = vec![];

    for (g, d) in input.split("\n\n").tuple_windows() {
        let rows = g.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();

        let row_count = rows.len();
        let col_count = rows[0].len();

        for r in 0..row_count {
            let mut acc = vec![];
            for c in 0..col_count {
                acc.push(match rows[r].get(c) {
                    Some(b'.') => 1,
                    Some(b'#') => 2,
                    _ => 0,
                });
            }
            grid.push(acc);
        }

        let dd = d
            .split(|c: char| c.is_ascii_alphabetic())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut dr = d
            .split(|c: char| c.is_ascii_digit())
            .filter(|x| !x.is_empty())
            .map(|x| x.as_bytes()[0])
            .collect::<Vec<_>>();

        dr.push(b'S');

        dirs = dd.into_iter().zip(dr.into_iter()).collect();
    }

    let mut row = 0 as i32;
    let mut col = 0;
    let mut dir = 0;
    for (i, v) in grid[0].iter().enumerate() {
        if v == &1 {
            col = i as i32;
            break;
        }
    }

    let face_coords = [(100, 50), (50, 50), (0, 50), (0, 100), (100, 0), (150, 0)];

    const BOTTOM_FACE: i32 = 0;
    const FRONT_FACE: i32 = 1;
    const TOP_FACE: i32 = 2;
    const RIGHT_FACE: i32 = 3;
    const LEFT_FACE: i32 = 4;
    const BACK_FACE: i32 = 5;

    const RIGHT_DIRECTION: i32 = 0;
    const DOWN_DIRECTION: i32 = 1;
    const LEFT_DIRECTION: i32 = 2;
    const UP_DIRECTION: i32 = 3;

    let map = [
        (
            BOTTOM_FACE,
            [
                (RIGHT_DIRECTION, (RIGHT_FACE, LEFT_DIRECTION, true)),
                (DOWN_DIRECTION, (BACK_FACE, LEFT_DIRECTION, false)),
                (LEFT_DIRECTION, (LEFT_FACE, LEFT_DIRECTION, false)),
                (UP_DIRECTION, (FRONT_FACE, UP_DIRECTION, false)),
            ],
        ),
        (
            FRONT_FACE,
            [
                (RIGHT_DIRECTION, (RIGHT_FACE, UP_DIRECTION, false)),
                (DOWN_DIRECTION, (BOTTOM_FACE, DOWN_DIRECTION, false)),
                (LEFT_DIRECTION, (LEFT_FACE, DOWN_DIRECTION, false)),
                (UP_DIRECTION, (TOP_FACE, UP_DIRECTION, false)),
            ],
        ),
        (
            TOP_FACE,
            [
                (RIGHT_DIRECTION, (RIGHT_FACE, RIGHT_DIRECTION, false)),
                (DOWN_DIRECTION, (FRONT_FACE, DOWN_DIRECTION, false)),
                (LEFT_DIRECTION, (LEFT_FACE, RIGHT_DIRECTION, true)),
                (UP_DIRECTION, (BACK_FACE, RIGHT_DIRECTION, false)),
            ],
        ),
        (
            RIGHT_FACE,
            [
                (RIGHT_DIRECTION, (BOTTOM_FACE, LEFT_DIRECTION, true)),
                (DOWN_DIRECTION, (FRONT_FACE, LEFT_DIRECTION, false)),
                (LEFT_DIRECTION, (TOP_FACE, LEFT_DIRECTION, false)),
                (UP_DIRECTION, (BACK_FACE, UP_DIRECTION, false)),
            ],
        ),
        (
            LEFT_FACE,
            [
                (RIGHT_DIRECTION, (BOTTOM_FACE, RIGHT_DIRECTION, false)),
                (DOWN_DIRECTION, (BACK_FACE, DOWN_DIRECTION, false)),
                (LEFT_DIRECTION, (TOP_FACE, RIGHT_DIRECTION, true)),
                (UP_DIRECTION, (FRONT_FACE, RIGHT_DIRECTION, false)),
            ],
        ),
        (
            BACK_FACE,
            [
                (RIGHT_DIRECTION, (BOTTOM_FACE, UP_DIRECTION, false)),
                (DOWN_DIRECTION, (RIGHT_FACE, DOWN_DIRECTION, false)),
                (LEFT_DIRECTION, (TOP_FACE, DOWN_DIRECTION, false)),
                (UP_DIRECTION, (LEFT_FACE, UP_DIRECTION, false)),
            ],
        ),
    ];

    for (face, edges) in map.clone() {
        for (dir, (new_face, new_dir, invert)) in edges {
            let inverse_dir = (dir + 2) % 4;
            let inverse_new_dir = (new_dir + 2) % 4;

            let (alt_new_face, alt_new_dir, alt_invert) =
                map[new_face as usize].1[inverse_new_dir as usize].1;

            assert!(face != new_face);
            assert!(alt_new_face == face);
            assert!(alt_new_dir == inverse_dir);
            assert!(invert == alt_invert);
        }
    }

    for (i, (step, d)) in dirs.into_iter().enumerate() {
        assert!(
            grid.get((row) as usize)
                .and_then(|v| v.get((col) as usize).map(|x| *x))
                .unwrap()
                == 1
        );

        if i <= 100 {
            for (r, rrow) in grid.iter().enumerate() {
                for (c, v) in rrow.iter().enumerate() {
                    if r == row as usize && c == col as usize {
                        eprint!("*");
                    } else if *v == 0 {
                        eprint!(" ");
                    } else if *v == 1 {
                        eprint!(".");
                    } else {
                        eprint!("#");
                    }
                }
                eprintln!();
            }
            eprintln!("================================================================================================================================================================================================================================================================================================================================================================================================================================================================================================================================================================================================================================================================================================================================================");
        }

        for _ in 0..step {
            let (dr, dc) = [(0, 1), (1, 0), (0, -1), (-1, 0)][dir as usize];

            let v = grid
                .get((row + dr) as usize)
                .and_then(|v| v.get((col + dc) as usize).map(|x| *x));

            // Hit Wall.
            if v.is_some() && v.unwrap() == 2 {
                continue;
            }

            // Open space.
            if v.is_some() && v.unwrap() == 1 {
                row += dr;
                col += dc;
                continue;
            }

            // Handle wrapping.
            let mut relative_position = 0;
            let (mut new_face, mut new_dir, mut invert) = (0, 0, false);
            for (face, (tr, tc)) in face_coords.iter().cloned().enumerate() {
                // Check right edge.
                if col == tc + 49 && row >= tr && row < tr + 50 && dc == 1 {
                    relative_position = row - tr;
                    (new_face, new_dir, invert) = map[face].1[0].1;
                    break;
                }

                // Check bottom edge.
                if row == tr + 49 && col >= tc && col < tc + 50 && dr == 1 {
                    relative_position = col - tc;
                    (new_face, new_dir, invert) = map[face].1[1].1;
                    break;
                }

                // Check left edge.
                if col == tc && row >= tr && row < tr + 50 && dc == -1 {
                    relative_position = row - tr;
                    (new_face, new_dir, invert) = map[face].1[2].1;
                    break;
                }

                // Check top edge.
                if row == tr && col >= tc && col < tc + 50 && dr == -1 {
                    relative_position = col - tc;
                    (new_face, new_dir, invert) = map[face].1[3].1;
                    break;
                }
            }

            if invert {
                relative_position = 49 - relative_position;
            }
            assert!(relative_position >= 0 && relative_position < 50);

            let (nr, nc) = match new_dir {
                // Going right
                0 => (
                    face_coords[new_face as usize].0 + relative_position,
                    face_coords[new_face as usize].1,
                ),
                // Going down
                1 => (
                    face_coords[new_face as usize].0,
                    face_coords[new_face as usize].1 + relative_position,
                ),
                // Going left
                2 => (
                    face_coords[new_face as usize].0 + relative_position,
                    face_coords[new_face as usize].1 + 49,
                ),
                3 => (
                    face_coords[new_face as usize].0 + 49,
                    face_coords[new_face as usize].1 + relative_position,
                ),
                _ => unreachable!(),
            };

            if grid[nr as usize][nc as usize] == 2 {
                continue;
            }
            row = nr;
            col = nc;
            dir = new_dir;
        }

        dir += if d == b'L' {
            -1
        } else if d == b'R' {
            1
        } else {
            0
        };
        if dir < 0 {
            dir += 4;
        }
        dir %= 4;
    }

    println!("{row} {col} {dir}");

    1000 * (row as i64 + 1) + 4 * (col as i64 + 1) + dir as i64
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
