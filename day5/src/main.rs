use std::fs;
use std::mem;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let input = input.trim();

    let _ = part1(input);
    let _ = part2(input);

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let result = react(input);
    println!("Part 1 result is {}", result.len());
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut min = input.len();
    for b in b'A'..=b'Z' {
        let unit1 = b as char;
        let unit2 = (b + 32) as char;
        let cleaned = input.replace(unit1, "").replace(unit2, "");
        let reacted = react(&cleaned);
        if reacted.len() < min {
            min = reacted.len();
        }
    }
    println!("Part 2 result is {}", min);

    Ok(())
}

fn react(polymer_str: &str) -> String {
    let mut polymer = polymer_str.as_bytes().to_vec();
    let mut vec = vec![];
    loop {
        let mut reacted = false;
        let mut i = 1;
        while i < polymer.len() {
            if is_react(polymer[i - 1], polymer[i]) {
                reacted = true;
                i += 2;
                continue;
            }
            vec.push(polymer[i - 1]);
            i += 1;
        }
        if i == polymer.len() {
            vec.push(polymer[i - 1]);
        }
        mem::swap(&mut polymer, &mut vec);
        vec.clear();
        if !reacted {
            break;
        }
    }
    String::from_utf8(polymer).unwrap()
}

fn is_react(b1: u8, b2: u8) -> bool {
    if b1 < b2 {
        b2 - b1 == 32
    } else {
        b1 - b2 == 32
    }
}
