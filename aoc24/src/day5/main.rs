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

fn order(a: u64, b: u64, orders: &Vec<(u64, u64)>) -> Ordering {
    if orders.contains(&(b, a)) {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

fn part1(input: &str) -> u64 {
    let (rules, lines) = input.split_once("\n\n").unwrap();
    let orders = rules
        .lines()
        .map(|l| {
            let (a, b) = l.split_once("|").unwrap();
            (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
        })
        .collect();
    lines
        .lines()
        .map(|l| {
            let pages: Vec<_> = l.split(",").map(|x| x.parse::<u64>().unwrap()).collect();
            let mut to_order = pages.clone();
            to_order.sort_by(|a, b| order(*a, *b, &orders));
            if pages == to_order {
                pages[pages.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let (rules, lines) = input.split_once("\n\n").unwrap();
    let orders = rules
        .lines()
        .map(|l| {
            let (a, b) = l.split_once("|").unwrap();
            (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
        })
        .collect();
    lines
        .lines()
        .map(|l| {
            let pages: Vec<_> = l.split(",").map(|x| x.parse::<u64>().unwrap()).collect();
            let mut to_order = pages.clone();
            to_order.sort_by(|a, b| order(*a, *b, &orders));
            if pages == to_order {
                0
            } else {
                to_order[to_order.len() / 2]
            }
        })
        .sum()
}
