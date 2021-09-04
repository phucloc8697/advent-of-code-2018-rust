use std::collections::HashSet;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let _ = part1(&input);
    let _ = part2(&input);
    Ok(())
}

fn part1(input: &String) -> Result<()> {
    let mut freq = 0;
    for line in input.lines() {
        let change: i32 = line.parse()?;
        freq += change;
    }
    println!("Part 1 result is {}", freq);

    Ok(())
}

fn part2(input: &String) -> Result<()> {
    let mut freq = 0;
    let mut set = HashSet::new();
    'outer: loop {
        for line in input.lines() {
            let change: i32 = line.parse()?;
            freq += change;
            if set.contains(&freq) {
                println!("Part 2 result is {}", freq);
                break 'outer;
            } else {
                set.insert(freq);
            }
        }
    }

    Ok(())
}
