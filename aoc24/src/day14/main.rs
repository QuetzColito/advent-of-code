use std::{io, time::Instant};

fn main() {
    let start_time = Instant::now();
    let input = include_str!("input.txt");
    let test = include_str!("test.txt");
    // println!("{}", part1(test));
    println!("{}", part1(input));
    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);

    let start_time = Instant::now();
    let input = include_str!("input.txt");
    let test = include_str!("test.txt");
    // println!("{}", part2(test));
    println!("{}", part2(input));

    let end_time = Instant::now();
    println!("{:?}", end_time - start_time);
}

fn parse(input: &str) -> ((i64, i64), (i64, i64)) {
    let mut split = input.split_whitespace();
    let pos = split
        .next()
        .unwrap()
        .split_once('=')
        .unwrap()
        .1
        .split_once(',')
        .unwrap();
    let vel = split
        .next()
        .unwrap()
        .split_once('=')
        .unwrap()
        .1
        .split_once(',')
        .unwrap();
    (
        (pos.0.parse().unwrap(), pos.1.parse().unwrap()),
        (vel.0.parse().unwrap(), vel.1.parse().unwrap()),
    )
}

fn mul(a: i64, (x, y): (i64, i64)) -> (i64, i64) {
    (a * x, a * y)
}

fn add((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> (i64, i64) {
    (x1 + x2, y1 + y2)
}

fn modu((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> (i64, i64) {
    (x1.rem_euclid(x2), y1.rem_euclid(y2))
}

fn abs((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn part1(input: &str) -> u64 {
    let mut quadrant_counts = vec![0; 4];
    let bounds = (101, 103);
    for line in input.lines() {
        let (pos, vel) = parse(line);
        let target = modu(add(pos, mul(100, vel)), bounds);
        if target.0 < bounds.0 / 2 {
            if target.1 < bounds.1 / 2 {
                quadrant_counts[0] += 1;
            }
            if target.1 > bounds.1 / 2 {
                quadrant_counts[1] += 1;
            }
        }
        if target.0 > bounds.0 / 2 {
            if target.1 < bounds.1 / 2 {
                quadrant_counts[2] += 1;
            }
            if target.1 > bounds.1 / 2 {
                quadrant_counts[3] += 1;
            }
        }
    }
    quadrant_counts.into_iter().product()
}

fn turn(
    robots: Vec<((i64, i64), (i64, i64))>,
    bounds: &(i64, i64),
) -> Vec<((i64, i64), (i64, i64))> {
    robots
        .into_iter()
        .map(|(pos, vel)| (modu(add(pos, vel), *bounds), vel))
        .collect()
}

fn view(robots: &Vec<((i64, i64), (i64, i64))>, bounds: &(i64, i64)) -> String {
    let mut s = String::new();
    for y in 0..bounds.1 {
        for x in 0..bounds.0 {
            s.push(if robots.iter().any(|(pos, _)| *pos == (x, y)) {
                '#'
            } else {
                '.'
            });
        }
        s.push('\n');
    }
    s
}

fn entropy(robots: &Vec<((i64, i64), (i64, i64))>, bounds: &(i64, i64)) -> i64 {
    robots
        .iter()
        // .map(|(pos, _)| robots.iter().map(|(pos2, _)| abs(*pos, *pos2)).sum::<i64>())
        .map(|(pos, _)| abs((bounds.0 / 2, bounds.1 / 2), *pos))
        .sum()
}

fn part2(input: &str) -> u64 {
    let bounds = (101, 103);
    let mut robots = input.lines().map(|l| parse(l)).collect();

    let mut min = i64::MAX;
    let mut current;
    let mut run = true;
    let mut s = String::new();
    let mut seconds = 0;
    while run {
        seconds += 1;
        robots = turn(robots, &bounds);
        current = entropy(&robots, &bounds);
        println!("{}", seconds);
        if current < min {
            min = current;
            println!("{}", view(&robots, &bounds));
            let _ = io::stdin().read_line(&mut s);
            if s.starts_with("q") {
                run = false
            }
        }
    }
    seconds
}
