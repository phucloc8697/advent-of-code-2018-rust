use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    println!("Hello, world!");

    part1()?;
    part2()?;

    Ok(())
}

fn part1() -> Result<()> {
    const PLAYERS: usize = 471;
    const LAST_MARBLES: u32 = 72026;

    let mut circle = Circle::new();
    let mut players = vec![Player::default(); PLAYERS];

    play(&mut players, &mut circle, LAST_MARBLES);
    let max = players.iter().map(|p| p.points).max().unwrap();
    println!("Part 1 result is {}", max);

    Ok(())
}

fn part2() -> Result<()> {
    const PLAYERS: usize = 471;
    const LAST_MARBLES: u32 = 7202600;

    let mut circle = Circle::new();
    let mut players = vec![Player::default(); PLAYERS];

    play(&mut players, &mut circle, LAST_MARBLES);
    let max = players.iter().map(|p| p.points).max().unwrap();
    println!("Part 1 result is {}", max);

    Ok(())
}

fn play(players: &mut [Player], circle: &mut Circle, marbles: u32) {
    let start = circle.max_marble_blue() + 1;
    let end = start + marbles;
    for (player_id, value) in (0..players.len()).cycle().zip(start..end) {
        circle.turn(&mut players[player_id], value);
    }
}

type MarbleID = usize;
type MarbleValue = u32;

#[derive(Clone, Debug, Default)]
struct Player {
    points: u32,
}

struct Marble {
    value: MarbleValue,
    prev: MarbleID,
    next: MarbleID,
}

impl Marble {
    fn unlinked(value: MarbleValue) -> Marble {
        Marble {
            value,
            prev: 0,
            next: 0,
        }
    }
}

struct Circle {
    marbles: Vec<Marble>,
    current: MarbleID,
}

impl Circle {
    fn new() -> Circle {
        let first = Marble {
            value: 0,
            prev: 0,
            next: 0,
        };
        Circle {
            marbles: vec![first],
            current: 0,
        }
    }

    fn add_marble(&mut self, value: MarbleValue) -> MarbleID {
        let id = self.marbles.len();
        self.marbles.push(Marble::unlinked(value));
        id
    }

    fn remove(&mut self, to_remove: MarbleID) {
        let (prev, next) = (self.marbles[to_remove].prev, self.marbles[to_remove].next);
        self.marbles[prev].next = next;
        self.marbles[next].prev = prev;
    }

    fn clockwise(&mut self, mut i: usize) -> MarbleID {
        let mut id = self.current;
        while i > 0 {
            id = self.marbles[id].next;
            i -= 1;
        }
        id
    }

    fn counter_clockwise(&mut self, mut i: usize) -> MarbleID {
        let mut id = self.current;
        while i > 0 {
            id = self.marbles[id].prev;
            i -= 1;
        }
        id
    }

    fn insert_after(&mut self, to_insert: MarbleID, after: MarbleID) {
        let old_next = self.marbles[after].next;
        self.marbles[after].next = to_insert;
        self.marbles[old_next].prev = to_insert;
        self.marbles[to_insert].prev = after;
        self.marbles[to_insert].next = old_next;
    }

    fn turn(&mut self, player: &mut Player, value: MarbleValue) {
        let marble_id = self.add_marble(value);
        if value % 23 != 0 {
            let insert_at = self.clockwise(1);
            self.insert_after(marble_id, insert_at);
            self.current = marble_id;
            return;
        }
        player.points += value;
        let remove_id = self.counter_clockwise(7);
        player.points += self.marbles[remove_id].value;
        self.remove(remove_id);
        self.current = self.counter_clockwise(6);
    }

    fn max_marble_blue(&self) -> MarbleValue {
        (self.marbles.len() - 1) as MarbleValue
    }
}
