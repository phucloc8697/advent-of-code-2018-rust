use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let pots: Pots = input.parse()?;

    run(&pots, 20);
    run(&pots, 500); // 19384
    run(&pots, 5000); // 190384
    run(&pots, 50000); // 1900384

    Ok(())
}

fn run(pots: &Pots, times: usize) {
    let mut new_pots = pots.clone();
    for _ in 0..times {
        new_pots = new_pots.step();
    }
    println!("Result after {} is {}", times, new_pots.sum_plant());
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Pot {
    Plant,
    Empty,
}

impl Pot {
    fn has_plant(&self) -> bool {
        *self == Pot::Plant
    }
}

impl FromStr for Pot {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Pot> {
        if s.is_empty() {
            err!("no pot in empty string")
        } else if &s[0..1] == "#" {
            Ok(Pot::Plant)
        } else if &s[0..1] == "." {
            Ok(Pot::Empty)
        } else {
            err!("unrecognized pot state: {:?}", s)
        }
    }
}

#[derive(Clone)]
struct Pots {
    state: HashMap<i32, Pot>,
    transitions: Vec<Transition>,
    min: i32,
    max: i32,
}

impl FromStr for Pots {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Pots> {
        let mut lines = s.lines();
        let first = match lines.next() {
            None => return err!("empty input for pots"),
            Some(first) => first,
        };

        let prefix = "initial state: ";
        if !first.starts_with(prefix) {
            return err!("unexpected prefix for first line: {:?}", first);
        }
        let pots: HashMap<i32, Pot> = first[prefix.len()..]
            .char_indices()
            .map(|(i, _)| s[prefix.len() + i..].parse())
            .collect::<Result<Vec<Pot>>>()?
            .into_iter()
            .enumerate()
            .map(|(i, pot)| (i as i32, pot))
            .collect();

        match lines.next() {
            None => return err!("missing empty line separating transitions"),
            Some(second) => {
                if !second.is_empty() {
                    return err!("second line is not empty: {:?}", second);
                }
            }
        }

        let transitions = lines
            .map(|line| line.parse())
            .collect::<Result<Vec<Transition>>>()?
            // Drop transitions to empty pots.
            .into_iter()
            .filter(|t| t.to.has_plant())
            .collect::<Vec<Transition>>();

        let (min, max) = (-2, pots.len() as i32 + 2);
        Ok(Pots {
            state: pots,
            transitions,
            min,
            max,
        })
    }
}

impl fmt::Debug for Pots {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in self.min..=self.max {
            if self.pot(i).has_plant() {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

impl Pots {
    fn sum_plant(&self) -> i32 {
        self.state
            .iter()
            .filter(|&(_, pot)| pot.has_plant())
            .map(|(&i, _)| i)
            .sum()
    }

    fn fresh(&self) -> Pots {
        Pots {
            state: HashMap::default(),
            transitions: self.transitions.clone(),
            min: self.min,
            max: self.max,
        }
    }

    fn step(&self) -> Pots {
        let mut new = self.fresh();
        for &i in self.state.keys() {
            for j in i - 2..=i + 2 {
                new.set_pot(j, self.next_state(&self.current_state(j)));
            }
        }
        new
    }

    fn pot(&self, i: i32) -> Pot {
        self.state.get(&i).map(|&pot| pot).unwrap_or(Pot::Empty)
    }

    fn set_pot(&mut self, i: i32, pot: Pot) {
        if pot.has_plant() {
            self.min = self.min.min(i - 2);
            self.max = self.max.max(i + 2);
            self.state.insert(i, pot);
        }
    }

    fn current_state(&self, at: i32) -> Vec<Pot> {
        let mut state = vec![];
        for i in at - 2..=at + 2 {
            state.push(self.pot(i));
        }
        state
    }

    fn next_state(&self, current: &[Pot]) -> Pot {
        for t in &self.transitions {
            if t.is_match(current) {
                return t.to;
            }
        }
        Pot::Empty
    }
}

#[derive(Clone, Debug)]
struct Transition {
    from: Vec<Pot>,
    to: Pot,
}

impl Transition {
    fn is_match(&self, state: &[Pot]) -> bool {
        self.from == state
    }
}

impl FromStr for Transition {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Transition> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<from>[#.]{5}) => (?P<to>[#.])$",).unwrap();
        }

        let caps = match RE.captures(s) {
            None => return err!("unrecognized transition"),
            Some(caps) => caps,
        };
        let from = caps["from"]
            .char_indices()
            .map(|(i, _)| s[i..].parse())
            .collect::<Result<Vec<Pot>>>()?;
        Ok(Transition {
            from: from,
            to: caps["to"].parse()?,
        })
    }
}
