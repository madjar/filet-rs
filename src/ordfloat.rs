#[deriving(PartialEq, PartialOrd)]
pub struct OrdFloat(pub f64);

impl Eq for OrdFloat {}

impl Ord for OrdFloat {
    fn cmp(&self, other: &OrdFloat) -> Ordering {
        if self < other { Less }
        else if self > other { Greater }
        else { Equal }
    }
}
