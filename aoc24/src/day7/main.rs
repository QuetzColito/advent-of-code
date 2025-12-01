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

fn calibrate(target: u64, position: usize, current: u64, values: &Vec<u64>) -> bool {
    let addcurr = current + values[position];
    let mulcurr = current * values[position];
    if position + 1 == values.len() {
        return target == addcurr || target == mulcurr;
    }
    if calibrate(target, position + 1, addcurr, values) {
        return true;
    } else {
        return calibrate(target, position + 1, mulcurr, values);
    }
}

fn concat(a: u64, b: u64) -> u64 {
    (10_u64.pow(b.ilog10() + 1) as u64) * a + b
}

fn calibrate2(target: u64, position: usize, current: u64, values: &Vec<u64>) -> bool {
    let addcurr = current + values[position];
    let mulcurr = current * values[position];
    let concatcurr = concat(current, values[position]);
    if position + 1 == values.len() {
        return target == addcurr || target == mulcurr || target == concatcurr;
    }
    if calibrate2(target, position + 1, addcurr, values) {
        true
    } else if calibrate2(target, position + 1, mulcurr, values) {
        true
    } else {
        calibrate2(target, position + 1, concatcurr, values)
    }
}

fn part1(input: &str) -> u64 {
    dbg!(concat(10, 12));
    input
        .lines()
        .map(|l| {
            let (result, values) = l.split_once(':').unwrap();
            let result = result.parse::<u64>().unwrap();
            let values = values
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            if calibrate(result, 1, values[0], &values) {
                result
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (result, values) = l.split_once(':').unwrap();
            let result = result.parse::<u64>().unwrap();
            let values = values
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            if calibrate2(result, 1, values[0], &values) {
                result
            } else {
                0
            }
        })
        .sum()
}
