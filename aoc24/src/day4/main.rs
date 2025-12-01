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

fn apply_offset(x: usize, n: i32, dx: &i32) -> usize {
    if (x as i32) + n * dx >= 0 {
        ((x as i32) + n * dx) as usize
    } else {
        usize::MAX
    }
}

fn part1(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut matches = 0;
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            matches += vec![
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ]
            .iter()
            .map(|(dx, dy)| {
                (0..4)
                    .map(|n| {
                        if let Some(row) = grid.get(apply_offset(y, n, dy)) {
                            row.get(apply_offset(x, n, dx)).unwrap_or(&'E')
                        } else {
                            &'E'
                        }
                    })
                    .collect::<String>()
            })
            .filter(|s| s == "XMAS")
            .count() as u64;
        }
    }
    matches
}

fn part2(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut matches = 0;
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            let corners = vec![(1, 1), (-1, 1), (-1, -1), (1, -1)]
                .iter()
                .map(|(dx, dy)| {
                    if let Some(row) = grid.get(apply_offset(y, 1, dy)) {
                        row.get(apply_offset(x, 1, dx)).unwrap_or(&'E')
                    } else {
                        &'E'
                    }
                })
                .collect::<String>();
            if grid[y][x] == 'A'
                && vec!["MMSS", "SMMS", "SSMM", "MSSM"]
                    .iter()
                    .any(|p| *p == corners)
            {
                matches += 1;
            }
        }
    }
    matches
}
