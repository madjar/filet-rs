use collections::Vec;
use std::cell::RefCell;

pub struct Bubble {
    pub x: f64,
    pub y: f64,
}

pub type Node = RefCell<Bubble>;

pub fn make_node<T: ToPrimitive>(x: T, y: T) -> Node {
    RefCell::new(Bubble {x: x.to_f64().unwrap(), y: y.to_f64().unwrap()})
}

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
