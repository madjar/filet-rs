use collections::Vec;
use std::cell::RefCell;

pub struct Bubble {
    pub x: f64,
    pub y: f64,
}

pub type Node = RefCell<Bubble>;

pub struct Graph<'a> {
    pub nodes: Vec<Node>,
    pub edges: Vec<(&'a Node, &'a Node)>,
}

impl<'a> Graph<'a> {
    pub fn from_fn(size: uint, op: |uint| -> Node) -> Graph {
        Graph {
            nodes: Vec::from_fn(size, op),
            edges: Vec::new(),
        }
    }
}
