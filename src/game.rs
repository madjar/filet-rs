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
use graph::{
    Graph,
    Node,
};
use std::rand::random;
use ordfloat::OrdFloat;

static RADIUS: f64 = 30.;

fn closest<'a>(graph: &'a Graph, x: f64, y: f64, threshold: f64) -> Option<&'a Node> {
    let distance = |n: &&Node| {
        let n = n.borrow();
        (n.x - x) * (n.x - x) + (n.y - y) * (n.y - y)
    };
    graph.nodes.iter()
        .min_by(|n| OrdFloat(distance(n)))
        .filtered(|n| distance(n) < threshold * threshold)
}

pub fn play<'a, W: GameWindow>(mut game_iter: GameIterator<'a, W>) {
    let ref mut gl = Gl::new();

    let mut selected: Option<&Node> = None;
    let mut mousex = 0.;
    let mut mousey = 0.;

    let mut graph = Graph::random(10, 500, 600);
    graph.edges = Graph::randomized_edges(&graph.nodes);

    loop {
        match game_iter.next() {
            None => { break },
            Some(e) => match e {
                Render(args) => {
                    gl.viewport(0, 0, args.width as i32, args.height as i32);
                    let c = Context::abs(args.width as f64, args.height as f64);
                    c.rgb(1.0, 1.0, 1.0).clear(gl);
                    for &(n1, n2) in graph.edges.iter() {
                        let n1 = n1.borrow();
                        let n2 = n2.borrow();
                        c.line(n1.x, n1.y, n2.x, n2.y)
                            .grey(0.)
                            .square_border_width(3.)
                            .stroke(gl);
                    }
                    for n in graph.nodes.iter() {
                        let n = n.borrow();
                        c.circle(n.x, n.y, RADIUS)
                            .rgb(1., 0., 0.)
                            .fill(gl);
                    }
                },
                MousePress(args) => {
                    if args.button == mouse::Left {
                        selected = closest(&graph, mousex, mousey, RADIUS);
                    } else if args.button == mouse::Middle {
                        graph.disperse();
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
                            let mut n = n.borrow_mut();
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
