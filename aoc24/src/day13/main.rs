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

fn parse(input: &str, delimiter: char) -> (i64, i64) {
    let (x, y) = input.split_once(':').unwrap().1.split_once(',').unwrap();
    (
        x.split_once(delimiter).unwrap().1.trim().parse().unwrap(),
        y.split_once(delimiter).unwrap().1.trim().parse().unwrap(),
    )
}

fn mul(a: i64, (x, y): (i64, i64)) -> (i64, i64) {
    (a * x, a * y)
}

fn add((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> (i64, i64) {
    (x1 + x2, y1 + y2)
}

fn part1(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|lines| {
            let mut lines = lines.lines();
            let a = parse(lines.next().unwrap(), '+');
            let b = parse(lines.next().unwrap(), '+');
            let target = parse(lines.next().unwrap(), '=');
            let mut count = 0;
            for xa in 0..101 {
                for xb in 0..101 {
                    if add(mul(xa, a), mul(xb, b)) == target && (count == 0 || 3 * xa + xb < count)
                    {
                        count = 3 * xa + xb;
                    }
                }
            }
            count as u64
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|lines| {
            let mut lines = lines.lines();
            let (xa, ya) = parse(lines.next().unwrap(), '+');
            let (xb, yb) = parse(lines.next().unwrap(), '+');
            let (xp, yp) = parse(lines.next().unwrap(), '=');
            let (xp, yp) = (xp + 10000000000000, yp + 10000000000000);
            let a = (xp * yb - yp * xb) / (xa * yb - ya * xb);
            let b = (xa * yp - ya * xp) / (xa * yb - ya * xb);
            if add(mul(a, (xa, ya)), mul(b, (xb, yb))) == (xp, yp) {
                (3 * a + b) as u64
            } else {
                0
            }
        })
        .sum()
}
