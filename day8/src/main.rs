use std::error::Error;
use std::fs;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let nums: Vec<u32> = input.split(" ").map(|s| s.parse().unwrap()).collect();
    let head = Node::new(&nums).unwrap();

    let _ = part1(&nums);
    let _ = part2(&head);
    Ok(())
}

fn part1(input: &Vec<u32>) -> Result<()> {
    let mut root = 0;
    let ans = get_sum_meta(input, &mut root).unwrap();
    println!("Part 1 result is {}", ans);
    Ok(())
}

fn part2(head: &Node) -> Result<()> {
    let ans = get_value(head).unwrap();
    println!("Part 2 result is {}", ans);
    Ok(())
}

fn get_sum_meta(input: &Vec<u32>, pos: &mut usize) -> Result<u32> {
    if *pos >= input.len() {
        return err!("index {} out of bound of length {}", pos, input.len());
    }
    let num_child = input[*pos];
    let num_entry = input[*pos + 1];
    let mut sum = 0;
    *pos = *pos + 2;
    for _ in 0..num_child {
        sum += get_sum_meta(input, pos).unwrap();
    }
    for _ in 0..num_entry {
        sum += input[*pos];
        *pos += 1;
    }
    Ok(sum)
}

fn get_value(node: &Node) -> Result<u32> {
    let mut sum = 0;
    if node.children.is_empty() {
        sum = node.entries.iter().sum();
    } else {
        for &entry in node.entries.iter() {
            if entry == 0 || entry > (node.children.len() as u32) {
                continue;
            }
            sum += get_value(&node.children[(entry - 1) as usize]).unwrap();
        }
    }

    Ok(sum)
}

#[derive(Debug)]
struct Node {
    head: usize,
    children: Vec<Node>,
    entries: Vec<u32>,
}

impl Node {
    fn new(input: &Vec<u32>) -> Result<Node> {
        if input.len() == 0 {
            return err!("empty input");
        }
        let mut pos: usize = 0;
        let head = create_node(input, &mut pos).unwrap();
        Ok(head)
    }
}

fn create_node(input: &Vec<u32>, pos: &mut usize) -> Result<Node> {
    if *pos >= input.len() {
        return err!("index {} out of bound of length {}", pos, input.len());
    }
    let index = *pos;
    let num_child = input[*pos];
    let num_entry = input[*pos + 1];
    let mut children: Vec<Node> = Vec::new();
    let mut entries: Vec<u32> = Vec::new();
    *pos = *pos + 2;
    for _ in 0..num_child {
        let node = create_node(input, pos).unwrap();
        children.push(node);
    }
    for _ in 0..num_entry {
        entries.push(input[*pos]);
        *pos += 1;
    }
    Ok(Node {
        head: index,
        children,
        entries,
    })
}
