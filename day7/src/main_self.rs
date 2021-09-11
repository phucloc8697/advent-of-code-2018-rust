use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::str::FromStr;

// step order: BFKEGNOVATIHXYZRMCJDLSUPWQ
// step order (part 2): BFKVEGAOTNYIHXZRMCJLDSUPWQ
// total seconds: 1020

macro_rules! err {
    ($($text:tt)*) => { Err(Box::<dyn Error>::from(format!($($text)*))) }
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let mut paths: Vec<Path> = Vec::new();
    for line in input.lines() {
        let path: Path = line
            .parse()
            .or_else(|err| err!("failed to parse '{:?}': {}", line, err))?;

        paths.push(path);
    }
    let graph = Graph::new(&paths).unwrap();
    let _ = part1(&graph);

    Ok(())
}

fn is_ready(graph: &Graph, visited: &HashSet<Node>, node: &Node) -> bool {
    if graph.dependencies.contains_key(node) {
        for c in graph.dependencies[&node].iter() {
            if !visited.contains(&c) {
                return false;
            }
        }
    }
    true
}

fn part1(graph: &Graph) -> Result<()> {
    let mut ans = String::new();
    let mut added: HashSet<Node> = HashSet::new();
    let mut visited: HashSet<Node> = HashSet::new();
    let mut queue: Vec<Node> = vec![];
    for root in graph.roots.iter() {
        queue.push(*root);
        added.insert(*root);
    }
    while !queue.is_empty() {
        queue.sort();
        let mut i = 0;
        while i < queue.len() {
            let c = queue[i];
            if is_ready(graph, &visited, &c) {
                if graph.adjacencies.contains_key(&c) {
                    for node in graph.adjacencies[&c].iter() {
                        if !added.contains(node) {
                            queue.push(*node);
                            added.insert(*node);
                        }
                    }
                }
                ans.push_str(&c.to_string());
                visited.insert(c);
                queue.remove(i);
                break;
            }
            i += 1;
        }
    }

    println!("Part 1 result is {}", ans);

    Ok(())
}

type Node = char;

#[derive(Debug)]
struct Path {
    from: Node,
    to: Node,
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path {} -> {}", self.from, self.to)
    }
}

impl FromStr for Path {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Path> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                Step\s
                (?P<from>[A-Z]+)
                (\s|[a-z])+
                (?P<to>[A-Z]+)
                (\s|[a-z])+\.
                "
            )
            .unwrap();
        }
        let caps = RE.captures(s).ok_or("unrecognized description")?;
        Ok(Path {
            from: caps["from"].parse()?,
            to: caps["to"].parse()?,
        })
    }
}

#[derive(Debug)]
struct Graph {
    roots: Vec<Node>,
    adjacencies: HashMap<Node, Vec<Node>>,
    dependencies: HashMap<Node, Vec<Node>>,
}

impl Graph {
    fn new(paths: &Vec<Path>) -> Result<Graph> {
        if paths.is_empty() {
            return err!("Paths empty");
        }
        let mut adjacencies: HashMap<Node, Vec<Node>> = HashMap::new();
        let mut dependencies: HashMap<Node, Vec<Node>> = HashMap::new();

        for path in paths {
            adjacencies.entry(path.from).or_default().push(path.to);
            dependencies.entry(path.to).or_default().push(path.from);
        }

        let mut roots: Vec<Node> = vec![];
        for (&key, value) in adjacencies.iter_mut() {
            value.sort();
            let mut is_root = true;
            for path in paths {
                if path.to == key {
                    is_root = false;
                    break;
                }
            }
            if is_root {
                roots.push(key);
            }
        }
        roots.sort();

        Ok(Graph {
            roots,
            adjacencies,
            dependencies,
        })
    }

    fn traverse_all(&self) -> Result<String> {
        let mut visited: HashSet<Node> = HashSet::new();
        let mut ans = String::new();
        for root in self.roots.iter() {
            println!("{}", root);
            let next = self.traverse(&root, &mut visited)?;
            ans.push_str(&next);
        }
        Ok(ans)
    }

    fn traverse(&self, begin: &Node, visited: &mut HashSet<Node>) -> Result<String> {
        let mut ans = begin.to_string();
        if !self.adjacencies.contains_key(begin) {
            return Ok(begin.to_string());
        }
        for node in self.adjacencies[begin].iter() {
            if !visited.contains(node) {
                visited.insert(*node);
                if let Ok(res) = self.traverse(&node, visited) {
                    ans.push_str(&res.to_string());
                }
            }
        }

        Ok(ans)
    }
}
