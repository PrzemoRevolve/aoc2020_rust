use crate::{first_answer, read_input_no_skip, second_answer, Input};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pixel {
    H,
    D,
}

type Edge = Vec<Pixel>;
type Data = Vec<Pixel>;
type TilesMap = HashMap<usize, Tile>;
struct Tile {
    id: usize,
    data: Data,
    size: usize,
    rotated: usize,
}

struct Node<'a> {
    tile: &'a Tile,
    left: Option<usize>,
    right: Option<usize>,
    top: Option<usize>,
    bottom: Option<usize>,
}

struct Edges {
    left: Edge,
    right: Edge,
    top: Edge,
    bottom: Edge,
}

impl Edges {
    fn into_array(&self) -> [&Edge; 4] {
        [&self.top, &self.right, &self.bottom, &self.left]
    }
}

impl<'a> Node<'a> {
    fn new(tile: &'a Tile) -> Node {
        Node {
            tile,
            left: None,
            right: None,
            top: None,
            bottom: None,
        }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pixel::D => ".",
                Pixel::H => "#",
            }
        )
    }
}

impl Tile {
    // edges[0] top->
    // edges[1] right \/
    // edges[2] bottom ->
    // edges[3] left \/
    fn get_edges(&self) -> Edges {
        let size = self.size;

        let top = self.data[..size].to_vec();
        let right = self
            .data
            .iter()
            .enumerate()
            .filter(|(i, _)| i % size == (size - 1))
            .map(|(_, d)| d.clone())
            .collect();
        let bottom = self.data[size * (size - 1)..].to_vec();
        let left = self
            .data
            .iter()
            .enumerate()
            .filter(|(i, _)| i % size == 0)
            .map(|(_, d)| d.clone())
            .collect();

        Edges {
            left,
            right,
            top,
            bottom,
        }
    }

    fn print_edges(&self) -> () {
        let edges = self.get_edges();

        for p in &edges.top {
            print!("{}", p)
        }

        print!("\n");

        for (p2, p1) in edges.left[1..9].iter().zip(edges.right[1..9].iter()) {
            println!("{}        {}", p1, p2);
        }
        for p in &edges.bottom {
            print!("{}", p)
        }

        print!("\n\n");
    }

    fn print(&self) -> () {
        for (i, p) in self.data.iter().enumerate() {
            print!("{}", p);

            if i % self.size == self.size - 1 {
                print!("\n");
            }
        }

        print!("\n");
    }

    fn rotate(&mut self, times: Option<usize>) {
        self.rotated += times.unwrap_or(1);
    }
}

pub fn run(day: u8) {
    let input = read_input_no_skip(day);

    first_answer(&first_problem(&input));
    second_answer(&second_problem(&input));
}

fn first_problem(input: &Input) -> String {
    let tiles = parse(input);
    let mut nodes: Vec<Node> = tiles.values().map(|t| Node::new(t)).collect();
    let mut current = nodes.pop().unwrap();

    for node in nodes.iter_mut() {
        if let None = current.top {
            if edges_match(&node.tile.get_edges().bottom, &current.tile.get_edges().top) {
                current.top = Some(node.tile.id);
                node.bottom = Some(current.tile.id);
            }
        }
        if edges_match(&node.tile.get_edges().left, &current.tile.get_edges().right) {
            current.right = Some(node.tile.id);
            node.left = Some(current.tile.id);
        }
        if edges_match(&node.tile.get_edges().top, &current.tile.get_edges().bottom) {
            current.bottom = Some(node.tile.id);
            node.top = Some(current.tile.id);
        }
        if edges_match(&node.tile.get_edges().right, &current.tile.get_edges().left) {
            current.left = Some(node.tile.id);
            node.right = Some(current.tile.id);
        }
    }

    String::from(format!("{:?}", "a"))
}

fn second_problem(input: &Input) -> String {
    String::from(format!("{:?}", "r42, r31"))
}

fn rotate_edges(edges: &Edges) -> Edges {
    Edges {
        top: edges.left.clone().into_iter().rev().collect(),
        bottom: edges.right.clone().into_iter().rev().collect(),
        left: edges.bottom.clone(),
        right: edges.top.clone(),
    }
}

fn parse(input: &Input) -> TilesMap {
    let mut map = HashMap::new();
    let mut tile = Tile {
        id: 0,
        data: vec![],
        rotated: 0,
        size: 0,
    };

    for line in input {
        if line.starts_with("Tile") {
            tile.id = line
                .split(" ")
                .nth(1)
                .unwrap()
                .split(":")
                .nth(0)
                .unwrap()
                .parse()
                .unwrap();
        } else if line.is_empty() {
            map.insert(tile.id, tile);

            tile = Tile {
                id: 0,
                data: vec![],
                rotated: 0,
                size: 0,
            }
        } else {
            tile.size = line.len();
            tile.data.extend(line.chars().map(|c| match c {
                '.' => Pixel::D,
                '#' => Pixel::H,
                p => panic!(format!("Unrecognized pixel {}", p)),
            }))
        }
    }

    map
}

fn edges_match(e1: &Edge, e2: &Edge) -> bool {
    e1.iter().zip(e2.iter()).all(|(p1, p2)| p1 == p2)
}
