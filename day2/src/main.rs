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
    let mut total_two = 0;
    let mut total_three = 0;
    for line in input.lines() {
        let texts: String = line.parse()?;
        let mut existed = HashSet::new();
        let mut twice = HashSet::new();
        let mut three = HashSet::new();
        let mut big = HashSet::new();
        for i in texts.chars() {
            if big.contains(&i) {
                continue;
            } else if !existed.contains(&i) {
                existed.insert(i);
            } else if three.contains(&i) {
                three.remove(&i);
                big.insert(i);
            } else if twice.contains(&i) {
                twice.remove(&i);
                three.insert(i);
            } else if existed.contains(&i) {
                twice.insert(i);
            }
        }
        if !twice.is_empty() {
            total_two += 1;
        }
        if !three.is_empty() {
            total_three += 1;
        }
    }
    println!(
        "Part 1 result is {} * {} = {}",
        total_two,
        total_three,
        total_two * total_three
    );

    Ok(())
}

fn part2(input: &String) -> Result<()> {
    let ids: Vec<&str> = input.lines().collect();
    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            if let Some(common) = common_correct_letters(&ids[i], &ids[j]) {
                println!("Part 2 result is {}", common);
                return Ok(());
            }
        }
    }

    Ok(())
}

fn common_correct_letters(str1: &str, str2: &str) -> Option<String> {
    if str1.len() != str2.len() {
        return None;
    }
    let mut one_wrong = false;
    for (c1, c2) in str1.chars().zip(str2.chars()) {
        if c1 != c2 {
            if one_wrong {
                return None;
            }
            one_wrong = true;
        }
    }
    let result = str1
        .chars()
        .zip(str2.chars())
        .filter(|&(c1, c2)| c1 == c2)
        .map(|(c, _)| c)
        .collect();
    Some(result)
}
