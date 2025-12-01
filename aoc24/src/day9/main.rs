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
    let mut data = Vec::new();
    let mut is_file = true;
    let mut file_id = 0;
    for c in input.chars().filter(|c| c.is_digit(10)) {
        let length = c.to_digit(10).unwrap();
        for _ in 0..length {
            if is_file {
                data.push(Some(file_id))
            } else {
                data.push(None)
            }
        }
        if is_file {
            file_id += 1;
        }
        is_file = !is_file
    }
    let mut pos = 0;
    data.clone().iter().enumerate().rev().for_each(|(i, item)| {
        if item.is_some() && i >= pos {
            while data[pos].is_some() {
                pos += 1;
            }
            data[pos] = Some(item.unwrap());
            data[i] = None;
        }
    });
    let filtered: Vec<_> = data.iter().filter_map(|x| *x).collect();
    filtered
        .iter()
        .enumerate()
        .map(|(i, v)| (i * v) as u64)
        .sum()
}

#[derive(Clone, Debug)]
enum Data {
    File(u64, u64),
    Free(u64),
}
use Data::*;

fn part2(input: &str) -> u64 {
    let mut data = Vec::new();
    let mut is_file = true;
    let mut file_id = 0;
    for c in input.chars().filter(|c| c.is_digit(10)) {
        let length = c.to_digit(10).unwrap() as u64;
        if is_file {
            data.push(File(length, file_id));
        } else {
            data.push(Free(length))
        }
        if is_file {
            file_id += 1;
        }
        is_file = !is_file
    }
    // dbg!(data.clone());
    let mut ordered = Vec::new();
    for i in 0..data.len() {
        match data[i] {
            File(a, b) => ordered.push(File(a, b)),
            Free(free_length) => {
                let mut free_length = free_length;
                for ir in (i..data.len()).rev() {
                    if let File(file_length, file_id) = data[ir] {
                        if file_length <= free_length {
                            ordered.push(File(file_length, file_id));
                            data[ir] = Free(file_length);
                            free_length = free_length - file_length;
                        }
                    }
                }
                if free_length > 0 {
                    ordered.push(Free(free_length))
                }
            }
        }
    }
    // dbg!(ordered.clone());
    ordered
        .iter()
        .flat_map(|d| match d {
            File(length, file_id) => vec![*file_id; *length as usize],
            Free(length) => vec![0; *length as usize],
        })
        .enumerate()
        .map(|(i, v)| i as u64 * v)
        .sum()
}
