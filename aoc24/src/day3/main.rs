use std::time::Instant;

use regex::Regex;

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
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    re.captures_iter(input)
        .map(|mul| mul.extract::<0>().0)
        .map(|s| {
            let (x, y) = s.split_once(',').unwrap();
            x.chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
                * y.chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap()
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0;
    let mut allow_mul = true;
    for m in re.captures_iter(input).map(|mul| mul.extract::<0>().0) {
        match m {
            "do()" => allow_mul = true,
            "don't()" => allow_mul = false,
            _ => {
                if allow_mul {
                    let (x, y) = m.split_once(',').unwrap();
                    sum += x
                        .chars()
                        .filter(|c| c.is_digit(10))
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap()
                        * y.chars()
                            .filter(|c| c.is_digit(10))
                            .collect::<String>()
                            .parse::<u64>()
                            .unwrap();
                }
            }
        }
    }
    sum
}
