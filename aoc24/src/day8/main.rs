use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

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

fn part1(input: &str) -> u64 {
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != '.' {
                map.entry(grid[y][x])
                    .or_insert(Vec::new())
                    .push((x as i32, y as i32));
            }
        }
    }
    let mut set = HashSet::new();
    for coords in map.values() {
        for (x1, y1) in coords {
            for (x2, y2) in coords {
                let xn = x2 + (x2 - x1);
                let yn = y2 + (y2 - y1);
                if !(x1 == x2 && y1 == y2)
                    && (xn >= 0)
                    && (xn < grid[0].len() as i32)
                    && (yn >= 0)
                    && (yn < grid.len() as i32)
                {
                    set.insert((xn, yn));
                }
            }
        }
    }
    set.len() as u64
}

fn part2(input: &str) -> u64 {
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != '.' {
                map.entry(grid[y][x])
                    .or_insert(Vec::new())
                    .push((x as i32, y as i32));
            }
        }
    }
    let mut set = HashSet::new();
    for coords in map.values() {
        for (x1, y1) in coords {
            for (x2, y2) in coords {
                let mut xn = *x2;
                let mut yn = *y2;
                while !(x1 == x2 && y1 == y2)
                    && (xn >= 0)
                    && (xn < grid[0].len() as i32)
                    && (yn >= 0)
                    && (yn < grid.len() as i32)
                {
                    set.insert((xn, yn));
                    xn = xn + (x2 - x1);
                    yn = yn + (y2 - y1);
                }
            }
        }
    }
    set.len() as u64
}
