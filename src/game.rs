
use graphics::*;
use piston::{
    GameIterator,
    GameWindow,
    MousePress,
    MouseRelease,
    MouseRelativeMove,
    MouseMove,
    Render,
};
use piston::mouse;
use std::cell::RefCell;
use graph::Graph;
use graph;

struct Bubble {
    x: f64,
    y: f64,
}

static RADIUS: f64 = 30.;

#[deriving(PartialEq, PartialOrd)]
struct OrdFloat(f64);

impl Eq for OrdFloat {}

impl Ord for OrdFloat {
    fn cmp(&self, other: &OrdFloat) -> Ordering {
        if self < other { Less }
        else if self > other { Greater }
        else { Equal }
    }
}

fn closest<'a>(nodes: &'a Graph<RefCell<Bubble>>, x: f64, y: f64, threshold: f64) -> Option<graph::Node<'a, RefCell<Bubble>>> {
    let distance = |n: &graph::Node<RefCell<Bubble>>| {
        let n = n.get().borrow();
        (n.x - x) * (n.x - x) + (n.y - y) * (n.y - y)
    };
    nodes.iter()
        .min_by(|n| OrdFloat(distance(n)))
        .filtered(|n| distance(n) < threshold * threshold)
}

pub fn play<'a, W: GameWindow>(mut game_iter: GameIterator<'a, W>) {
    let mut selected: Option<graph::Node<RefCell<Bubble>>> = None;
    let mut mousex = 0.;
    let mut mousey = 0.;

    let graph = Graph::from_fn(10, |i| RefCell::new(Bubble {
            x: (i as f64) * 50.,
            y: (i as f64) * 60.,
        }));
    
    loop {
        match game_iter.next() {
            None => { break },
            Some(e) => match e {
                Render(args) => {
                    let c = Context::abs(args.width as f64, args.height as f64);
                    for n in graph.iter() {
                        let n = n.get().borrow();
                        c.circle(n.x, n.y, RADIUS).rgb(0.1, 0.8, 0.8).fill(args.gl);
                    }
                },
                MousePress(args) => {
                    if args.button == mouse::Left {
                        selected = closest(&graph, mousex, mousey, RADIUS);
                    }
                }
                MouseRelease(args) => {
                    if args.button == mouse::Left {
                        selected = None;
                    }
                }
                MouseRelativeMove(args) => {
                    match selected {
                        Some(ref n) => {
                            let mut n = n.get().borrow_mut();
                            n.x += args.dx;
                            n.y += args.dy;
                        }
                        None => {}
                    };
                }
                MouseMove(args) => {
                    mousex = args.x;
                    mousey = args.y;
                }
                _ => {},
            }
        }        
    }

}
