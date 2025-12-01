use std::{cmp::Ordering, time::Instant};

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

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}
use Direction::*;

fn get(x: usize, y: usize, grid: &Vec<Vec<bool>>) -> Option<bool> {
    if let Some(row) = grid.get(y) {
        row.get(x).copied()
    } else {
        None
    }
}

fn dec(x: usize) -> usize {
    if x == 0 {
        usize::MAX
    } else {
        x - 1
    }
}

fn part1(input: &str) -> u64 {
    let (mut x, mut y) = (0, 0);
    let grid: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(ye, l)| {
            l.chars()
                .enumerate()
                .map(|(xe, c)| match c {
                    '#' => false,
                    '^' => {
                        (x, y) = (xe, ye);
                        true
                    }
                    _ => true,
                })
                .collect()
        })
        .collect();
    let mut visited: Vec<Vec<Vec<Direction>>> = (0..grid.len())
        .map(|row| (0..grid[row].len()).map(|_| Vec::new()).collect())
        .collect();
    let mut direction = NORTH;
    loop {
        // println!("({}, {})", x, y);
        // dbg!(direction);
        visited[y][x].push(direction);

        if let Some(has_next) = match direction {
            NORTH => get(x, dec(y), &grid),
            EAST => get(x + 1, y, &grid),
            SOUTH => get(x, y + 1, &grid),
            WEST => get(dec(x), y, &grid),
        } {
            if has_next {
                match direction {
                    NORTH => y = y - 1,
                    EAST => x = x + 1,
                    SOUTH => y = y + 1,
                    WEST => x = x - 1,
                }
            } else {
                direction = match direction {
                    NORTH => EAST,
                    EAST => SOUTH,
                    SOUTH => WEST,
                    WEST => NORTH,
                }
            }
        } else {
            return visited.iter().flatten().filter(|v| v.len() > 0).count() as u64;
        };
    }
}

fn part2(input: &str) -> u64 {
    let (mut x, mut y) = (0, 0);
    let mut count = 0;
    let mut grid: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(ye, l)| {
            l.chars()
                .enumerate()
                .map(|(xe, c)| match c {
                    '#' => false,
                    '^' => {
                        (x, y) = (xe, ye);
                        true
                    }
                    _ => true,
                })
                .collect()
        })
        .collect();
    for yo in 0..grid.len() {
        for xo in 0..grid[y].len() {
            let mut x = x;
            let mut y = y;
            if grid[yo][xo] {
                grid[yo][xo] = false;
                let mut visited: Vec<Vec<Vec<Direction>>> = (0..grid.len())
                    .map(|row| (0..grid[row].len()).map(|_| Vec::new()).collect())
                    .collect();
                let mut direction = NORTH;
                loop {
                    if visited[y][x].contains(&direction) {
                        count += 1;
                        break;
                    }
                    visited[y][x].push(direction);

                    if let Some(has_next) = match direction {
                        NORTH => get(x, dec(y), &grid),
                        EAST => get(x + 1, y, &grid),
                        SOUTH => get(x, y + 1, &grid),
                        WEST => get(dec(x), y, &grid),
                    } {
                        if has_next {
                            match direction {
                                NORTH => y = y - 1,
                                EAST => x = x + 1,
                                SOUTH => y = y + 1,
                                WEST => x = x - 1,
                            }
                        } else {
                            direction = match direction {
                                NORTH => EAST,
                                EAST => SOUTH,
                                SOUTH => WEST,
                                WEST => NORTH,
                            }
                        }
                    } else {
                        break;
                    };
                }
                grid[yo][xo] = true;
            }
        }
    }
    count
}
