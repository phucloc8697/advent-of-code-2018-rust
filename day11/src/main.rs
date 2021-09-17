use std::collections::HashMap;

type Grid = [[i32; 300]; 300];
type StoredSquarePower = HashMap<(usize, usize, usize), i32>;

fn main() {
    const SERIAL_NUMBER: i32 = 1788;

    let mut grid = [[0 as i32; 300]; 300];
    let mut dp: StoredSquarePower = HashMap::new();

    for y in 1..=300 {
        for x in 1..=300 {
            grid[y - 1][x - 1] = calculate_cell_power(x, y, SERIAL_NUMBER);
        }
    }
    part1(&grid, &mut dp);
    part2(&grid, &mut dp);
}

fn part1(grid: &Grid, dp: &mut StoredSquarePower) {
    let mut max = grid[0][0];
    let mut coord = (1, 1);
    for y in 1..=(301 - 3) {
        for x in 1..=(301 - 3) {
            let sum = calculate_square_power(&grid, dp, x, y, 3);
            if max < sum {
                max = sum;
                coord = (x, y);
            }
        }
    }
    println!("Part 1 result is {},{},{}", coord.0, coord.1, max);
}

fn part2(grid: &Grid, dp: &mut StoredSquarePower) {
    let mut max = grid[0][0];
    let mut coord = (1, 1, 1);
    for size in 1..=300 {
        for y in 1..=(301 - size) {
            for x in 1..=(301 - size) {
                let sum = calculate_square_power(&grid, dp, x, y, size);
                // println!("x={} y={} s=size{} sum={}", x, y, size, sum);
                if max < sum {
                    max = sum;
                    coord = (x, y, size);
                }
            }
        }
    }
    println!(
        "Part 2 result is {},{},{},{}",
        coord.0, coord.1, coord.2, max
    );
}

fn calculate_cell_power(x: usize, y: usize, serial: i32) -> i32 {
    let rack_id = (x + 10) as i32;
    let mut pow: i32 = rack_id * y as i32;
    pow = (pow + serial) * rack_id;
    pow = pow / 10 / 10 % 10;
    pow - 5
}

fn calculate_square_power(
    grid: &Grid,
    dp: &mut StoredSquarePower,
    x: usize,
    y: usize,
    size: usize,
) -> i32 {
    if size == 1 {
        *dp.entry((x, y, size)).or_default() = grid[y - 1][x - 1];
        return grid[y - 1][x - 1];
    }
    if dp.contains_key(&(x, y, size)) {
        return dp[&(x, y, size)];
    }
    let mut sum = grid[y - 1][x - 1];
    for dy in 1..size {
        sum += grid[y + dy - 1][x - 1];
    }
    for dx in 1..size {
        sum += grid[y - 1][x + dx - 1];
    }
    sum += calculate_square_power(grid, dp, x + 1, y + 1, size - 1);
    *dp.entry((x, y, size)).or_default() = sum;
    sum as i32
}
