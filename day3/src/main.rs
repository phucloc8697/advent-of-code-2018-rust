use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type Grid = HashMap<(u32, u32), u32>;

fn main() -> Result<()> {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let mut grid: Grid = HashMap::new();
    let mut claims: Vec<Claim> = Vec::new();
    for line in input.lines() {
        let claim: Claim = line.parse()?;
        for (x, y) in claim.iter() {
            *grid.entry((x, y)).or_default() += 1;
        }
        claims.push(claim);
    }

    part1(&grid);
    part2(&grid, &claims);

    Ok(())
}

fn part1(grid: &Grid) {
    let count = grid.values().filter(|&&count| count > 1).count();
    println!("Part 1 result is {}", count);
}

fn part2(grid: &Grid, claims: &Vec<Claim>) {
    for claim in claims {
        if claim.iter().all(|p| grid[&p] == 1) {
            println!("Part 2 result is {}", claim.id);
            return;
        }
    }
    println!("No uncontested claim");
}

#[derive(Debug)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl FromStr for Claim {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Claim> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                \#
                (?P<id>[0-9]+)
                \s+@\s+
                (?P<x>[0-9]+),(?P<y>[0-9]+):
                \s+
                (?P<width>[0-9]+)x(?P<height>[0-9]+)
            "
            )
            .unwrap();
        }
        let caps = RE.captures(s).ok_or("unrecognized claim")?;
        Ok(Claim {
            id: caps["id"].parse()?,
            x: caps["x"].parse()?,
            y: caps["y"].parse()?,
            width: caps["width"].parse()?,
            height: caps["height"].parse()?,
        })
    }
}

impl Claim {
    fn iter(&self) -> Point {
        Point {
            claim: self,
            px: self.x,
            py: self.y,
        }
    }
}

struct Point<'c> {
    claim: &'c Claim,
    px: u32,
    py: u32,
}

impl<'c> Iterator for Point<'c> {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<(u32, u32)> {
        if self.py > self.claim.y + self.claim.height {
            self.py = self.claim.y;
            self.px += 1;
        }
        if self.px >= self.claim.x + self.claim.width {
            return None;
        }
        let (px, py) = (self.px, self.py);
        self.py += 1;
        Some((px, py))
    }
}
