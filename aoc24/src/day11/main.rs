use std::{collections::HashMap, time::Instant};

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

fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::new();
    for (s, amount) in stones {
        if s == 0 {
            new_stones.insert(1, new_stones.get(&1).unwrap_or(&0) + amount);
        } else if s.ilog10() % 2 == 1 {
            let exp = 10_u64.pow((s.ilog10() + 1) / 2);
            let leading = s / exp;
            new_stones.insert(leading, new_stones.get(&leading).unwrap_or(&0) + amount);
            new_stones.insert(
                s - leading * exp,
                new_stones.get(&(s - leading * exp)).unwrap_or(&0) + amount,
            );
        } else {
            new_stones.insert(s * 2024, new_stones.get(&(s * 2024)).unwrap_or(&0) + amount);
        }
    }
    new_stones
}

fn part1(input: &str) -> u64 {
    let mut stones: HashMap<u64, u64> = input
        .split_whitespace()
        .map(|s| (s.parse().unwrap(), 1))
        .collect();
    for _ in 0..25 {
        stones = blink(stones);
    }
    stones.values().sum()
}

fn part2(input: &str) -> u64 {
    let mut stones: HashMap<u64, u64> = input
        .split_whitespace()
        .map(|s| (s.parse().unwrap(), 1))
        .collect();
    for _ in 0..75 {
        stones = blink(stones);
    }
    stones.values().sum()
}
