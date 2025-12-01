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

fn get((x, y): (usize, usize), grid: &Vec<Vec<u64>>) -> Option<u64> {
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

fn find_trail((x, y): (usize, usize), grid: &Vec<Vec<u64>>) -> Vec<(usize, usize)> {
    let current = grid[y][x];
    if current == 9 {
        vec![(x, y)]
    } else {
        vec![(x + 1, y), (dec(x), y), (x, y + 1), (x, dec(y))]
            .iter()
            .flat_map(|coords| {
                let next = get(*coords, grid).unwrap_or(20);
                if current + 1 == next {
                    find_trail(*coords, grid)
                } else {
                    Vec::new()
                }
            })
            .collect()
    }
}

fn part1(input: &str) -> u64 {
    let grid: Vec<Vec<u64>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        .collect();
    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 0 {
                let mut heads = find_trail((x, y), &grid);
                heads.sort();
                heads.dedup();
                count += heads.len();
            }
        }
    }
    count as u64
}

fn part2(input: &str) -> u64 {
    let grid: Vec<Vec<u64>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        .collect();
    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 0 {
                let mut heads = find_trail((x, y), &grid);
                count += heads.len();
            }
        }
    }
    count as u64
}
