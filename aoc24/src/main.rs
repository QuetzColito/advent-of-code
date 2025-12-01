use std::time::Instant;

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

use Direction::*;
#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn peek((x, y): (usize, usize), direction: Direction, grid: &Vec<Vec<bool>>) -> bool {
    match direction {
        North => grid[y - 1][x],
        East => grid[y][x + 1],
        South => grid[y + 1][x],
        West => grid[y][x - 1],
    }
}

fn step((x, y): (usize, usize), direction: Direction) -> (usize, usize) {
    match direction {
        North => (x, y - 1),
        East => (x + 1, y),
        South => (x, y + 1),
        West => (x - 1, y),
    }
}

fn nb(direction: Direction) -> (Direction, Direction) {
    match direction {
        North | South => (East, West),
        East | West => (North, South),
    }
}

fn part1(input: &str) -> u64 {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => false,
                    '.' | 'S' | 'E' => true,
                    _ => unimplemented!(""),
                })
                .collect()
        })
        .collect();
    let start = (1, grid[1].len() - 2);
    let end = (grid.len() - 2, 1);
    let mut min = u64::MAX;
    let mut runners = vec![(start, East, 0, Vec::new())];
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut minhist = Vec::new();
    while runners.len() > 0 {
        runners = runners
            .into_iter()
            .flat_map(|(pos, d, score, hist)| {
                if pos == end || score > min || visited[pos.1][pos.0] {
                    if pos == end && score < min {
                        minhist = hist;
                        min = score;
                    }
                    return Vec::new();
                }
                visited[pos.1][pos.0] = true;

                let mut next = Vec::new();
                if peek(pos, d, &grid) {
                    let mut hist = hist.clone();
                    hist.push(d);
                    next.push((step(pos, d), d, score + 1, hist));
                }

                let (n1, n2) = nb(d);
                if peek(pos, n1, &grid) {
                    let mut hist = hist.clone();
                    hist.push(n1);
                    next.push((step(pos, n1), n1, score + 1001, hist));
                }
                if peek(pos, n2, &grid) {
                    let mut hist = hist.clone();
                    hist.push(n2);
                    next.push((step(pos, n2), n2, score + 1001, hist));
                }
                next
            })
            .collect();
    }
    dbg!(minhist);
    min
}

fn part2(input: &str) -> u64 {
    2
}
