use std::{collections::HashSet, time::Instant};

fn main() {
    let start_time = Instant::now();
    let input = include_str!("input.txt");
    let test = include_str!("test.txt");
    println!("{}", part1(test));
    println!("{}", part1(input));
    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);

    let start_time = Instant::now();
    let input = include_str!("input.txt");
    let test = include_str!("test.txt");
    println!("{}", part2(test));
    println!("{}", part2(input));

    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);
}

#[derive(Clone, Copy, PartialEq)]
enum Tile1 {
    Empty,
    Box,
    Wall,
    Robot,
}

#[derive(Clone, Copy, Debug)]
enum Direction1 {
    North,
    East,
    South,
    West,
}

fn step1((x, y): (usize, usize), direction: Direction1) -> (usize, usize) {
    match direction {
        Direction1::North => (x, y - 1),
        Direction1::East => (x + 1, y),
        Direction1::South => (x, y + 1),
        Direction1::West => (x - 1, y),
    }
}

fn get1((x, y): (usize, usize), grid: &Vec<Vec<Tile1>>) -> Tile1 {
    grid[y][x]
}

fn peek1(point: (usize, usize), direction: Direction1, grid: &Vec<Vec<Tile1>>) -> Tile1 {
    get1(step1(point, direction), grid)
}

fn print1(grid: &Vec<Vec<Tile1>>) {
    let s: String = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|t| match t {
                    Tile1::Wall => '#',
                    Tile1::Empty => '.',
                    Tile1::Robot => '@',
                    Tile1::Box => 'O',
                })
                .collect::<String>()
                + "\n"
        })
        .collect();
    println!("{}", s);
}

fn part1(input: &str) -> u64 {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut pos = (0, 0);
    let mut grid: Vec<Vec<_>> = grid
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Tile1::Wall,
                    'O' => Tile1::Box,
                    '.' => Tile1::Empty,
                    '@' => {
                        pos = (x, y);
                        Tile1::Robot
                    }
                    _ => unimplemented!(""),
                })
                .collect()
        })
        .collect();

    let moves: Vec<_> = moves
        .lines()
        .flat_map(|l| {
            l.chars()
                .map(|c| match c {
                    '^' => Direction1::North,
                    '>' => Direction1::East,
                    'v' => Direction1::South,
                    '<' => Direction1::West,
                    _ => unimplemented!(""),
                })
                .collect::<Vec<_>>()
        })
        .collect();
    for m in moves {
        // dbg!(m);
        // print(&grid);
        // dbg!(pos);
        assert!(get1(pos, &grid) == Tile1::Robot, "eeeeeeeeeeeee");
        let mut origin = pos;
        let mut to_move = vec![(pos, Tile1::Robot)];
        while peek1(pos, m, &grid) == Tile1::Box {
            pos = step1(pos, m);
            to_move.push((pos, Tile1::Box))
        }
        if peek1(pos, m, &grid) == Tile1::Empty {
            grid[origin.1][origin.0] = Tile1::Empty;
            origin = step1(origin, m);
            for (p, t) in to_move.iter() {
                let (x, y) = step1(*p, m);
                grid[y][x] = *t;
            }
        }
        pos = origin;
    }
    grid.into_iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(|(x, i)| if i == Tile1::Box { 100 * y + x } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>() as u64
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Tile {
    Empty,
    LBox,
    RBox,
    Wall,
    Robot,
}
use Tile::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

fn step((x, y): (usize, usize), direction: Direction) -> (usize, usize) {
    match direction {
        North => (x, y - 1),
        East => (x + 1, y),
        South => (x, y + 1),
        West => (x - 1, y),
    }
}

fn get((x, y): (usize, usize), grid: &Vec<Vec<Tile>>) -> Tile {
    grid[y][x]
}

fn peek(point: (usize, usize), direction: Direction, grid: &Vec<Vec<Tile>>) -> Tile {
    get(step(point, direction), grid)
}

fn print(grid: &Vec<Vec<Tile>>) {
    let s: String = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|t| match t {
                    Wall => '#',
                    Empty => '.',
                    Robot => '@',
                    LBox => '[',
                    RBox => ']',
                })
                .collect::<String>()
                + "\n"
        })
        .collect();
    println!("{}", s);
}

fn part2(input: &str) -> u64 {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut pos = (0, 0);
    let mut grid: Vec<Vec<_>> = grid
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .flat_map(|(x, c)| match c {
                    '#' => vec![Wall, Wall],
                    'O' => vec![LBox, RBox],
                    '.' => vec![Empty, Empty],
                    '@' => {
                        pos = (2 * x, y);
                        vec![Robot, Empty]
                    }
                    _ => unimplemented!(""),
                })
                .collect()
        })
        .collect();

    let moves: Vec<_> = moves
        .lines()
        .flat_map(|l| {
            l.chars()
                .map(|c| match c {
                    '^' => North,
                    '>' => East,
                    'v' => South,
                    '<' => West,
                    _ => unimplemented!(""),
                })
                .collect::<Vec<_>>()
        })
        .collect();
    for m in moves {
        assert!(get(pos, &grid) == Robot, "eeeeeeeeeeeee");
        if m == East || m == West {
            let mut origin = pos;
            let mut to_move = vec![(pos, Robot)];
            while peek(pos, m, &grid) == LBox || peek(pos, m, &grid) == RBox {
                pos = step(pos, m);
                to_move.push((pos, get(pos, &grid)));
                pos = step(pos, m);
                to_move.push((pos, get(pos, &grid)));
            }
            if peek(pos, m, &grid) == Empty {
                grid[origin.1][origin.0] = Empty;
                origin = step(origin, m);
                for (p, t) in to_move.iter() {
                    let (x, y) = step(*p, m);
                    grid[y][x] = *t;
                }
            }
            pos = origin;
        } else {
            let mut origin = pos;
            let mut front = HashSet::new();
            front.insert(pos);
            let mut to_move = vec![(pos, Robot)];
            let mut found_box = true;
            while found_box {
                found_box = false;
                front = front
                    .iter()
                    .flat_map(|pos| {
                        let pos = *pos;
                        if peek(pos, m, &grid) == LBox || peek(pos, m, &grid) == RBox {
                            found_box = true;
                            let fst = step(pos, m);
                            to_move.push((fst, get(fst, &grid)));
                            let snd = step(fst, if get(fst, &grid) == LBox { East } else { West });
                            to_move.push((snd, get(snd, &grid)));
                            vec![fst, snd]
                        } else {
                            vec![pos]
                        }
                    })
                    .collect();
            }
            if !front.iter().any(|t| peek(*t, m, &grid) == Wall) {
                origin = step(origin, m);
                for ((x, y), _) in to_move.iter() {
                    grid[*y][*x] = Empty;
                }
                for (p, t) in to_move.iter() {
                    let (x, y) = step(*p, m);
                    grid[y][x] = *t;
                }
            }
            pos = origin;
        }
    }
    grid.into_iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(|(x, i)| if i == LBox { 100 * y + x } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>() as u64
}
