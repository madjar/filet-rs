use collections::Vec;
use std::cell::RefCell;
use std::rand::random;

pub struct Bubble {
    pub x: f64,
    pub y: f64,
}

pub type Node = RefCell<Bubble>;

pub fn make_node(x: f64, y: f64) -> Node {
    RefCell::new(Bubble {x: x, y: y})
}

pub struct Graph<'a> {
    pub nodes: Vec<Node>,
    pub edges: Vec<(&'a Node, &'a Node)>,
}

impl<'a> Graph<'a> {
    pub fn random(size: uint, height: uint, width: uint) -> Graph {
        Graph {
            nodes: Vec::from_fn(size, |_| make_node((height as f64) * random(),
                                                    (width as f64) * random())),
            edges: Vec::new(),
        }
    }

    pub fn randomized_edges(nodes: &'a Vec<Node>) -> Vec<(&'a Node, &'a Node)>{
        let mut edges = Vec::new();
        for i in range(0, 10u) {
            for j in range(i + 1, 10u) {
                if random::<f64>() < 0.3 {
                    edges.push((nodes.get(i), nodes.get(j)));
                }
            }
        }
        edges
    }

    pub fn disperse(&self) {
        
    }
}
