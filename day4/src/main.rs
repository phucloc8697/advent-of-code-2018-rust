use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::ops::Range;
use std::slice;
use std::str::FromStr;

macro_rules! err {
    ($($text:tt)*) => { Err(Box::<dyn Error>::from(format!($($text)*))) }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let mut events: Vec<Event> = Vec::new();
    for line in input.lines() {
        let event: Event = line
            .parse()
            .or_else(|err| err!("failed to parse '{:?}': {}", line, err))?;
        events.push(event);
    }
    if events.is_empty() {
        return err!("no events");
    }

    events.sort_by(|a, b| a.time.cmp(&b.time));
    let mut cur_guard = None;
    let mut guard_events = GuardEvents::new();
    for ev in events {
        if let EventKind::Begin { id } = ev.kind {
            cur_guard = Some(id);
        }
        match cur_guard {
            None => return err!("no guard for event"),
            Some(id) => {
                guard_events.entry(id).or_default().push(ev);
            }
        }
    }

    let mut minutes_asleep: GuardSleepMinute = HashMap::new();
    for (&id, events) in guard_events.iter() {
        let mut freq: [u32; 60] = [0; 60];
        for result in MinutesAsleepIter::new(events) {
            for minute in result? {
                freq[minute as usize] += 1;
            }
        }
        minutes_asleep.insert(id, freq);
    }

    let _ = part1(&minutes_asleep);
    let _ = part2(&minutes_asleep);
    Ok(())
}

fn part1(minutes_asleep: &GuardSleepMinute) -> Result<()> {
    let (&sleepiest, _) = minutes_asleep
        .iter()
        .max_by_key(|&(_, ref freqs)| -> u32 { freqs.iter().sum() })
        .unwrap();
    let _ = match sleepiest_minute(minutes_asleep, sleepiest) {
        Some(s) => {
            println!("Part 1 result is {}", s * sleepiest);
        }
        None => {
            return err!("guard {} never sleep", sleepiest);
        }
    };
    Ok(())
}

fn part2(minutes_asleep: &GuardSleepMinute) -> Result<()> {
    let mut sleepiest_minutes: HashMap<u32, (u32, u32)> = HashMap::new();
    for (&id, freqs) in minutes_asleep.iter() {
        let minute = match sleepiest_minute(minutes_asleep, id) {
            None => continue,
            Some(minute) => minute,
        };
        let count = freqs[minute as usize];
        sleepiest_minutes.insert(id, (minute, count));
    }
    if sleepiest_minutes.is_empty() {
        return err!("no guards slept");
    }
    let (&longest, &(minute, _)) = sleepiest_minutes
        .iter()
        .max_by_key(|&(_, (_, count))| count)
        .unwrap();
    println!("Part 2 result is {}", longest * minute);

    Ok(())
}

fn sleepiest_minute(minutes_asleep: &GuardSleepMinute, guard_id: u32) -> Option<u32> {
    let (sleepiest, _) = minutes_asleep[&guard_id]
        .iter()
        .enumerate()
        .max_by_key(|&(_, ref freqs)| -> u32 { **freqs })
        .expect("iterator of sleepy minutes should not be empty");
    Some(sleepiest as u32)
}

#[derive(Debug)]
enum EventKind {
    Fall,
    Wake,
    Begin { id: u32 },
}

#[derive(Debug)]
struct Event {
    time: DateTime,
    kind: EventKind,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct DateTime {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

type GuardEvents = HashMap<u32, Vec<Event>>;

type GuardSleepMinute = HashMap<u32, [u32; 60]>;

struct MinutesAsleepIter<'a> {
    events: slice::Iter<'a, Event>,
    fell_asleep: Option<u32>,
}

impl<'a> MinutesAsleepIter<'a> {
    fn new(events: &'a [Event]) -> MinutesAsleepIter<'a> {
        MinutesAsleepIter {
            events: events.iter(),
            fell_asleep: None,
        }
    }
}

impl<'a> Iterator for MinutesAsleepIter<'a> {
    type Item = Result<Range<u32>>;

    fn next(&mut self) -> Option<Result<Range<u32>>> {
        loop {
            let ev = match self.events.next() {
                Some(ev) => ev,
                None => {
                    if self.fell_asleep.is_some() {
                        return Some(err!("found sleep event without wake up"));
                    }
                    return None;
                }
            };
            match ev.kind {
                EventKind::Begin { .. } => {}
                EventKind::Fall => {
                    self.fell_asleep = Some(ev.time.minute);
                }
                EventKind::Wake => {
                    let fell_asleep = match self.fell_asleep.take() {
                        Some(minute) => minute,
                        None => {
                            return Some(err!("found wakeup without sleep"));
                        }
                    };
                    if ev.time.minute < fell_asleep {
                        return Some(err!("wake up before sleep"));
                    }
                    return Some(Ok(fell_asleep..ev.time.minute));
                }
            }
        }
    }
}

impl FromStr for Event {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Event> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                \[
                    (?P<year>[0-9]{4})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})
                    \s+
                    (?P<hour>[0-9]{2}):(?P<minute>[0-9]{2})
                \]
                \s+
                (?:Guard\ \#(?P<id>[0-9]+)\ begins\ shift|(?P<sleep>.+))
            "
            )
            .unwrap();
        }
        let caps = match RE.captures(s) {
            None => return err!("unrecognize event"),
            Some(caps) => caps,
        };
        let datetime = DateTime {
            year: caps["year"].parse()?,
            month: caps["month"].parse()?,
            day: caps["day"].parse()?,
            hour: caps["hour"].parse()?,
            minute: caps["minute"].parse()?,
        };
        let kind = if let Some(m) = caps.name("id") {
            EventKind::Begin {
                id: m.as_str().parse()?,
            }
        } else if &caps["sleep"] == "falls asleep" {
            EventKind::Fall
        } else if &caps["sleep"] == "wakes up" {
            EventKind::Wake
        } else {
            return err!("could not determind event kind");
        };

        Ok(Event {
            time: datetime,
            kind,
        })
    }
}
