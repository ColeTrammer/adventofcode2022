use clap::Parser;
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::rc::Rc;

struct Node {
    children: HashMap<String, Rc<RefCell<Node>>>,
    size: i32,
    dir: bool,
    parent: Option<Rc<RefCell<Node>>>,
}

fn part_a(input: String) -> i32 {
    let root = Rc::new(RefCell::new(Node {
        children: HashMap::new(),
        size: 0,
        dir: true,
        parent: None,
    }));

    let mut node = root.clone();

    let lines = input.lines().collect::<Vec<_>>();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if let Some(next) = line.strip_prefix("$ cd ") {
            if next == "/" {
                node = root.clone()
            } else if next == ".." {
                node = node.clone().borrow().parent.clone().unwrap().clone();
            } else if let Some(next) = node.clone().borrow().children.get(next).clone() {
                node = next.clone();
            } else {
                println!("???: {}", next);
            }
            i += 1;
        } else {
            i += 1;
            while i < lines.len() && !lines[i].starts_with("$") {
                let line = lines[i];
                let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
                let name = parts[1];
                if let Ok(size) = parts[0].parse::<i32>() {
                    node.borrow_mut().children.insert(
                        name.to_owned(),
                        Rc::new(RefCell::new(Node {
                            children: HashMap::new(),
                            size,
                            dir: false,
                            parent: Some(node.clone()),
                        })),
                    );
                } else {
                    node.borrow_mut().children.insert(
                        name.to_owned(),
                        Rc::new(RefCell::new(Node {
                            children: HashMap::new(),
                            size: 0,
                            dir: true,
                            parent: Some(node.clone()),
                        })),
                    );
                }
                i += 1;
            }
        }
    }

    fn dfs(node: Rc<RefCell<Node>>) -> i32 {
        let size = node
            .borrow()
            .children
            .iter()
            .map(|(_, v)| v)
            .map(|n| dfs(n.clone()))
            .sum::<i32>();
        node.borrow_mut().size += size;
        node.borrow_mut().size
    }

    dfs(root.clone());

    fn dfs2(node: Rc<RefCell<Node>>) -> Vec<Rc<RefCell<Node>>> {
        let mut result = vec![];
        if node.borrow().dir {
            for child in node.borrow().children.values() {
                let mut other = dfs2(child.clone());
                result.append(&mut other);
            }
            result.push(node.clone());
        }
        result
    }

    let dirs = dfs2(root.clone());
    dirs.iter()
        .filter(|dir| dir.borrow().size <= 100000)
        .map(|dir| dir.borrow().size)
        .sum()
}

fn part_b(input: String) -> i32 {
    let root = Rc::new(RefCell::new(Node {
        children: HashMap::new(),
        size: 0,
        dir: true,
        parent: None,
    }));

    let mut node = root.clone();

    let lines = input.lines().collect::<Vec<_>>();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if let Some(next) = line.strip_prefix("$ cd ") {
            if next == "/" {
                node = root.clone()
            } else if next == ".." {
                node = node.clone().borrow().parent.clone().unwrap().clone();
            } else if let Some(next) = node.clone().borrow().children.get(next).clone() {
                node = next.clone();
            } else {
                println!("???: {}", next);
            }
            i += 1;
        } else {
            i += 1;
            while i < lines.len() && !lines[i].starts_with("$") {
                let line = lines[i];
                let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
                let name = parts[1];
                if let Ok(size) = parts[0].parse::<i32>() {
                    node.borrow_mut().children.insert(
                        name.to_owned(),
                        Rc::new(RefCell::new(Node {
                            children: HashMap::new(),
                            size,
                            dir: false,
                            parent: Some(node.clone()),
                        })),
                    );
                } else {
                    node.borrow_mut().children.insert(
                        name.to_owned(),
                        Rc::new(RefCell::new(Node {
                            children: HashMap::new(),
                            size: 0,
                            dir: true,
                            parent: Some(node.clone()),
                        })),
                    );
                }
                i += 1;
            }
        }
    }

    fn dfs(node: Rc<RefCell<Node>>) -> i32 {
        let size = node
            .borrow()
            .children
            .iter()
            .map(|(_, v)| v)
            .map(|n| dfs(n.clone()))
            .sum::<i32>();
        node.borrow_mut().size += size;
        node.borrow_mut().size
    }

    dfs(root.clone());

    fn dfs2(node: Rc<RefCell<Node>>) -> Vec<Rc<RefCell<Node>>> {
        let mut result = vec![];
        if node.borrow().dir {
            for child in node.borrow().children.values() {
                let mut other = dfs2(child.clone());
                result.append(&mut other);
            }
            result.push(node.clone());
        }
        result
    }

    let mut dirs = dfs2(root.clone())
        .into_iter()
        .map(|x| x.borrow().size)
        .collect::<Vec<_>>();
    dirs.sort();

    let unused_space = 70000000 - root.borrow().size;
    *dirs
        .iter()
        .find_or_first(|x| **x >= 30000000 - unused_space)
        .unwrap()
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

    let input = read_to_string(args.input.unwrap_or("input.txt".into()))?;

    if args.part_b {
        println!("Part B: {:?}", part_b(input.clone()));
    } else {
        println!("Part A: {:?}", part_a(input.clone()));
    }

    Ok(())
}
