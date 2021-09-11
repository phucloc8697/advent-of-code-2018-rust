use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let input = input.trim();

    let coordinates = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Coordinate>>>()?;

    if coordinates.is_empty() {
        return Err(From::from("no coordinates given"));
    }

    let mut grid = Grid::new(coordinates);
    grid.find_finite();

    let _ = part1(&grid);
    let _ = part2(&grid);

    Ok(())
}

fn part1(grid: &Grid) -> Result<()> {
    let mut biggest_area = 0;
    for &loc in &grid.finite {
        let mut candidate_area = 0;
        for &loc2 in grid.table.values() {
            if loc == loc2 {
                candidate_area += 1;
            }
        }
        if candidate_area > biggest_area {
            biggest_area = candidate_area;
        }
    }
    println!("Part 1 result is {}", biggest_area);
    Ok(())
}

fn part2(grid: &Grid) -> Result<()> {
    let bound = 500;
    let mut size = 0;
    for x in -bound..=bound {
        for y in -bound..=bound {
            if grid.distance_sum(Coordinate { x, y }) < 10000 {
                size += 1;
            }
        }
    }
    println!("Part 2 result is {}", size);
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn distance(self, other: Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn border(self, step: i32) -> impl Iterator<Item = Coordinate> {
        (self.x - step..=self.x + step)
            .flat_map(move |x| (self.y - step..=self.y + step).map(move |y| Coordinate { x, y }))
            .filter(move |&c2| self.distance(c2) == step)
    }
}

impl FromStr for Coordinate {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Coordinate> {
        let comma = match s.find(",") {
            None => return Err(From::from("could not find comma")),
            Some(i) => i,
        };
        let (pos1, pos2) = (s[..comma].trim(), s[comma + 1..].trim());
        Ok(Coordinate {
            x: pos1.parse()?,
            y: pos2.parse()?,
        })
    }
}

#[derive(Debug)]
struct Grid {
    locations: Vec<Coordinate>,
    finite: HashSet<Coordinate>,
    table: HashMap<Coordinate, Coordinate>,
}

impl Grid {
    fn new(locations: Vec<Coordinate>) -> Grid {
        assert!(!locations.is_empty());
        Grid {
            locations,
            finite: HashSet::new(),
            table: HashMap::new(),
        }
    }

    fn distance_sum(&self, c: Coordinate) -> i32 {
        self.locations.iter().map(|&loc| loc.distance(c)).sum()
    }
    fn closest_location(&self, c: Coordinate) -> Option<Coordinate> {
        let (mut min, mut unique) = (self.locations[0], true);
        for &loc in &self.locations[1..] {
            if loc.distance(c) == min.distance(c) {
                unique = false;
            } else if loc.distance(c) < min.distance(c) {
                min = loc;
                unique = true;
            }
        }
        if !unique {
            None
        } else {
            Some(min)
        }
    }

    fn find_finite(&mut self) {
        for step in 0..100 {
            for loc in &self.locations {
                if self.finite.contains(&loc) {
                    continue;
                }
                for c in loc.border(step) {
                    let closest = match self.closest_location(c) {
                        None => continue,
                        Some(closest) => closest,
                    };
                    self.table.insert(c, closest);
                }
            }
            for &loc in &self.locations {
                if !loc.border(step).any(|c| self.table.get(&c) == Some(&loc)) {
                    self.finite.insert(loc);
                }
            }
        }
    }
}
