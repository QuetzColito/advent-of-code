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

fn part1(input: &str) -> u64 {
    let (mut list1, mut list2): (Vec<u64>, Vec<u64>) = input
        .lines()
        .map(str::split_whitespace)
        .map(|mut i| {
            (
                i.next().unwrap().parse::<u64>().unwrap(),
                i.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .unzip();

    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2.into_iter())
        .map(|(x, y)| x.abs_diff(y))
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut map1: HashMap<u64, u64> = HashMap::new();
    let mut map2: HashMap<u64, u64> = HashMap::new();

    for line in input.lines() {
        let mut line = line.split_whitespace();
        let i = line.next().unwrap().parse::<u64>().unwrap();
        map1.insert(i, map1.get(&i).unwrap_or(&0) + 1);
        let i = line.next().unwrap().parse::<u64>().unwrap();
        map2.insert(i, map2.get(&i).unwrap_or(&0) + 1);
    }

    map1.iter()
        .map(|(k, v)| v * k * map2.get(k).unwrap_or(&0))
        .sum()
}
