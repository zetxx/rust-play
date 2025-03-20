use std::primitive::i32;

struct Node {
    item: i32,
    next: PList
}

enum PList {
    Empty,
    Occupied(Box<Node>),
}

pub struct List {
    link: PList
}

fn main() {
}
