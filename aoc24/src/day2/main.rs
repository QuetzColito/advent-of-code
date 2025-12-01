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

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace().map(|i| i.parse::<u64>().unwrap());
            let mut x = it.next().unwrap();
            let mut y = it.next().unwrap();
            let ascending = x < y;
            let mut safe = x.abs_diff(y) < 4 && x != y;

            for i in it {
                x = y;
                y = i;
                if x.abs_diff(y) > 3 || x == y || ((x < y) != ascending) {
                    safe = false;
                    break;
                }
            }
            safe
        })
        .filter(|x| *x)
        .count() as u64
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace().map(|i| i.parse::<u64>().unwrap());
            let mut x = it.next().unwrap();
            let mut y = it.next().unwrap();
            let ascending = x < y;
            let mut safe = true;
            let mut dampener_used = false;

            if x.abs_diff(y) > 3 || x == y {
                dampener_used = true;
            } else {
                x = y;
            }
            for i in it {
                y = i;
                if x.abs_diff(y) > 3 || x == y || ((x < y) != ascending) {
                    if !dampener_used {
                        dampener_used = true;
                    } else {
                        safe = false;
                        break;
                    }
                } else {
                    x = y;
                }
            }
            // dbg!(safe);
            // dbg!(l);
            safe || {
                let mut it = l.split_whitespace().map(|i| i.parse::<u64>().unwrap());
                it.next();
                let mut x = it.next().unwrap();
                let mut y = it.next().unwrap();
                let ascending = x < y;
                let mut safe = x.abs_diff(y) < 4 && x != y;

                for i in it {
                    x = y;
                    y = i;
                    if x.abs_diff(y) > 3 || x == y || ((x < y) != ascending) {
                        safe = false;
                        break;
                    }
                }
                safe
            }
        })
        .filter(|x| *x)
        .count() as u64
}
