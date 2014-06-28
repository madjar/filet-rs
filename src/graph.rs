use collections::Vec;
use std::cell::RefCell;
use std::rand;
use ordfloat::OrdFloat;
use std::iter::{
    NoElements,
    OneElement,
    MinMax
};
use std::cell::Ref;
use game::RADIUS;

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
    width: f64,
    height: f64,
}

impl<'a> Graph<'a> {
    pub fn random(size: uint, width: f64, height: f64) -> Graph {
        Graph {
            nodes: Vec::from_fn(size, |_| make_node(height * rand::random(),
                                                    width * rand::random())),
            edges: Vec::new(),
            width: width,
            height: height,
        }
    }

    pub fn randomized_edges(nodes: &'a Vec<Node>) -> Vec<(&'a Node, &'a Node)>{
        let mut edges = Vec::new();
        for i in range(0, nodes.len()) {
            for j in range(i + 1, nodes.len()) {
                if rand::random::<f64>() < 0.3 {
                    edges.push((nodes.get(i), nodes.get(j)));
                }
            }
        }
        edges
    }

    pub fn disperse(&self) {
        let min_max_by = |f: |Ref<Bubble>| -> f64| {
            let (OrdFloat(min), OrdFloat(max)) =
                match self.nodes.iter()
                .map(|n| OrdFloat(f(n.borrow())))
                .min_max() {
                    MinMax(a, b) => (a, b),
                    OneElement(a) => (a, a),
                    NoElements => (OrdFloat(0.), OrdFloat(0.)),
                };
            (min, max)
        };

        let (xmin, xmax) = min_max_by(|n| n.x);
        let (ymin, ymax) = min_max_by(|n| n.y);

        let r = RADIUS as f64;
        for n in self.nodes.iter() {
            let mut n = n.borrow_mut();
            n.x = (n.x - xmin) / (xmax - xmin) * (self.width - r*2.) + r;
            n.y = (n.y - ymin) / (ymax - ymin) * (self.height - r*2.) + r;
        }
    }
}
