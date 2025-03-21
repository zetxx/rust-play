use std::primitive::i32;

struct Node {
    item: i32,
    next: Link
}

enum Link {
    Empty,
    Occupied(Box<Node>),
}

pub struct List {
    link: Link
}

impl List {
    pub fn new() -> Self {
        List { link: Link::Empty }
    }
}
fn main() {
}
