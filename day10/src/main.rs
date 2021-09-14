use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let mut points: Vec<Point> = Vec::new();
    for line in input.lines() {
        let p = line.parse()?;
        points.push(p);
    }
    let mut points = Points::new(points)?;
    for _ in 0..1_000_000 {
        points.run_1_step();
        let (w, h) = points.dimensions();
        if w <= 80 && h <= 80 {
            println!("Seconds: {}", points.seconds);
            println!("{}", points.grid_string().trim());
            println!("{}", "~".repeat(79));
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Point> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                position=<\s*(?P<x>[-0-9]+),\s*(?P<y>[-0-9]+)>
                \s+
                velocity=<\s*(?P<vx>[-0-9]+),\s*(?P<vy>[-0-9]+)>
            "
            )
            .unwrap();
        }
        let caps = match RE.captures(s) {
            None => return err!("unrecognize position or velocity"),
            Some(caps) => caps,
        };
        Ok(Point {
            x: caps["x"].parse()?,
            y: caps["y"].parse()?,
            vx: caps["vx"].parse()?,
            vy: caps["vy"].parse()?,
        })
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Bounds {
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
}

impl Bounds {
    fn distance_x(&self, x: i32) -> u32 {
        match self.minx >= 0 {
            true => (x - self.minx) as u32,
            false => (x + self.minx.abs()) as u32,
        }
    }

    fn distance_y(&self, y: i32) -> u32 {
        match self.miny >= 0 {
            true => (y - self.miny) as u32,
            false => (y + self.miny.abs()) as u32,
        }
    }

    fn width(&self) -> usize {
        (self.maxx - self.minx + 1) as usize
    }

    fn height(&self) -> usize {
        (self.maxy - self.miny + 1) as usize
    }
}

#[derive(Clone, Debug)]
struct Points {
    points: Vec<Point>,
    seconds: u32,
}

impl Points {
    fn new(points: Vec<Point>) -> Result<Points> {
        if points.is_empty() {
            err!("no points given")
        } else {
            Ok(Points { points, seconds: 0 })
        }
    }

    fn run_1_step(&mut self) {
        for p in &mut self.points {
            p.x += p.vx;
            p.y += p.vy;
        }
        self.seconds += 1;
    }

    fn bounds(&self) -> Bounds {
        let mut b = Bounds {
            minx: self.points[0].x,
            maxx: self.points[0].x,
            miny: self.points[0].y,
            maxy: self.points[0].y,
        };
        for p in &self.points {
            b.minx = b.minx.min(p.x);
            b.maxx = b.maxx.max(p.x);
            b.miny = b.miny.min(p.y);
            b.maxy = b.maxy.max(p.y);
        }
        b
    }

    fn dimensions(&self) -> (usize, usize) {
        let b = self.bounds();
        (b.width(), b.height())
    }

    fn grid_string(&self) -> String {
        let bounds = self.bounds();
        let mut grid = vec![vec![b'.'; bounds.width()]; bounds.height()];
        for p in &self.points {
            let x = bounds.distance_x(p.x);
            let y = bounds.distance_y(p.y);
            grid[y as usize][x as usize] = b'#';
        }

        let mut buffer = String::new();
        for row in grid {
            buffer.push_str(std::str::from_utf8(&row).unwrap());
            buffer.push('\n');
        }
        buffer
    }
}
