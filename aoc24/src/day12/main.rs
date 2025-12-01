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

#[derive(Debug)]
struct Area {
    circumference: u64,
    area: u64,
    symbol: char,
}

fn get((x, y): (usize, usize), grid: &Vec<Vec<char>>) -> Option<char> {
    if let Some(row) = grid.get(y) {
        row.get(x).copied()
    } else {
        None
    }
}

fn dec(x: usize) -> usize {
    if x == 0 {
        usize::MAX
    } else {
        x - 1
    }
}

fn part1(input: &str) -> u64 {
    let mut areas: Vec<Area> = Vec::new();
    let mut area_map: HashMap<(usize, usize), usize> = HashMap::new();
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut area_id = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if !visited[y][x] {
                visited[y][x] = true;
                let mut area = 1;
                let mut to_check = vec![(x, y)];
                area_map.insert((x, y), area_id);
                let symbol = grid[y][x];
                while let Some((x, y)) = to_check.pop() {
                    for (x1, y1) in vec![(x + 1, y), (dec(x), y), (x, y + 1), (x, dec(y))] {
                        let point = (x1, y1);
                        if get((x1, y1), &grid).unwrap_or('\n') == symbol && !visited[y1][x1] {
                            visited[y1][x1] = true;
                            area_map.insert(point, area_id);
                            area += 1;
                            to_check.push(point);
                        }
                    }
                }
                areas.push(Area {
                    circumference: 0,
                    area,
                    symbol,
                });
                area_id += 1;
            }
        }
    }
    // dbg!(area_map.clone());
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if x == 0 || x + 1 == grid[y].len() {
                areas[*area_map.get(&(x, y)).unwrap()].circumference += 1;
            }
            if y == 0 || y + 1 == grid.len() {
                areas[*area_map.get(&(x, y)).unwrap()].circumference += 1;
            }
            if x + 1 != grid[y].len() {
                if grid[y][x + 1] != grid[y][x] {
                    areas[*area_map.get(&(x, y)).unwrap()].circumference += 1;
                    areas[*area_map.get(&(x + 1, y)).unwrap()].circumference += 1;
                }
            }
            if y + 1 != grid.len() {
                if grid[y + 1][x] != grid[y][x] {
                    areas[*area_map.get(&(x, y)).unwrap()].circumference += 1;
                    areas[*area_map.get(&(x, y + 1)).unwrap()].circumference += 1;
                }
            }
        }
    }
    areas.into_iter().map(|a| a.area * a.circumference).sum()
}

fn part2(input: &str) -> u64 {
    let mut areas: Vec<Area> = Vec::new();
    let mut area_map: HashMap<(usize, usize), usize> = HashMap::new();
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut area_id = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if !visited[y][x] {
                visited[y][x] = true;
                let mut area = 1;
                let mut to_check = vec![(x, y)];
                area_map.insert((x, y), area_id);
                let symbol = grid[y][x];
                while let Some((x, y)) = to_check.pop() {
                    for (x1, y1) in vec![(x + 1, y), (dec(x), y), (x, y + 1), (x, dec(y))] {
                        let point = (x1, y1);
                        if get((x1, y1), &grid).unwrap_or('\n') == symbol && !visited[y1][x1] {
                            visited[y1][x1] = true;
                            area_map.insert(point, area_id);
                            area += 1;
                            to_check.push(point);
                        }
                    }
                }
                areas.push(Area {
                    circumference: 0,
                    area,
                    symbol,
                });
                area_id += 1;
            }
        }
    }
    // dbg!(area_map.clone());
    for y in 0..grid.len() {
        let end = grid[y].len() - 1;
        if grid[y][0] != get((0, dec(y)), &grid).unwrap_or('\n') {
            areas[*area_map.get(&(0, y)).unwrap()].circumference += 1;
            // dbg!(grid[y][0]);
            // dbg!(y);
            // dbg!(0);
        }
        if grid[y][end] != get((end, dec(y)), &grid).unwrap_or('\n') {
            areas[*area_map.get(&(end, y)).unwrap()].circumference += 1;
            // dbg!(grid[y][end]);
            // dbg!(y);
            // dbg!(end);
        }
        dbg!("forward");
        for x in 0..end {
            if grid[y][x + 1] != grid[y][x]
                && (grid[y][x] != get((x, dec(y)), &grid).unwrap_or('\n')
                    || grid[y][x] == get((x + 1, dec(y)), &grid).unwrap_or('\n'))
            {
                // dbg!(grid[y][x]);
                // dbg!(y);
                // dbg!(x);
                areas[*area_map.get(&(x, y)).unwrap()].circumference += 1;
            }
        }
        dbg!("backward");
        for x in 1..grid[y].len() {
            if grid[y][x - 1] != grid[y][x]
                && (grid[y][x] != get((x, dec(y)), &grid).unwrap_or('\n')
                    || grid[y][x] == get((x - 1, dec(y)), &grid).unwrap_or('\n'))
            {
                areas[*area_map.get(&(x, y)).unwrap()].circumference += 1;
                // dbg!(grid[y][x]);
                // dbg!(y);
                // dbg!(x);
            }
        }
    }
    for x in 0..grid[0].len() {
        let end = grid.len() - 1;
        if grid[0][x] != get((dec(x), 0), &grid).unwrap_or('\n') {
            areas[*area_map.get(&(x, 0)).unwrap()].circumference += 1;
        }
        if grid[end][x] != get((dec(x), end), &grid).unwrap_or('\n') {
            areas[*area_map.get(&(x, end)).unwrap()].circumference += 1;
        }
        for y in 0..end {
            if grid[y + 1][x] != grid[y][x]
                && (grid[y][x] != get((dec(x), y), &grid).unwrap_or('\n')
                    || grid[y][x] == get((dec(x), y + 1), &grid).unwrap_or('\n'))
            {
                areas[*area_map.get(&(x, y)).unwrap()].circumference += 1;
            }
        }
        for y in 1..grid.len() {
            if grid[y - 1][x] != grid[y][x]
                && (grid[y][x] != get((dec(x), y), &grid).unwrap_or('\n')
                    || grid[y][x] == get((dec(x), y - 1), &grid).unwrap_or('\n'))
            {
                areas[*area_map.get(&(x, y)).unwrap()].circumference += 1;
            }
        }
    }
    dbg!(&areas);
    areas.into_iter().map(|a| a.area * a.circumference).sum()
}
