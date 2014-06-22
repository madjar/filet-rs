use collections::Vec;

pub struct Graph<T> {
    graph: Vec<bool>,
    size: uint,
    values: Vec<T>,
}

struct Iter<'a, T> {
    graph: &'a Graph<T>,
    index: uint,
}

pub struct Node<'a, T> {
    graph: &'a Graph<T>,
    index: uint,
}

#[inline]
fn ordered(n: uint, m: uint) -> (uint, uint) {
    if n > m {
        (m, n)
    } else {
        (n, m)
    }
}

impl<T> Graph<T> {
    pub fn from_fn(size: uint, op: |uint| -> T) -> Graph<T> {
        Graph {
            graph: Vec::from_elem((size * (size - 1)) / 2, false),
            size: size,
            values: Vec::from_fn(size, op),
        }
    }

    #[inline]
    fn arc_index(&self, n: uint, m: uint) -> uint {
        let (n, m) = ordered(n, m);
        n*(self.size-1) - (n*(n-1))/2 + (m - n - 1)
}        

    pub fn has_arc(&self, n: uint, m: uint) -> bool {
        if n == m {
            false
        } else {
            *self.graph.get(self.arc_index(n, m))
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { graph: self, index: 0}
    }
}


impl<'a, T> Iterator<Node<'a, T>> for Iter<'a, T> {
    fn next(&mut self) -> Option<Node<'a, T>> {
        self.index += 1;
        if self.index < self.graph.size {
            Some(Node { graph: self.graph, index: self.index -1})
        } else {
            None
        }
    }
}

impl<'a, T> Node<'a, T> {
    pub fn get(&self) -> &'a T {
        self.graph.values.get(self.index)
    }
}
