use std::primitive::i32;
use std::{i32, mem};

struct Node {
    elem: i32,
    next: Link
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub struct List {
    head: Link
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, el: i32) {
        let new_node = Box::new(Node {
            elem: el,
            next: mem::replace(&mut self.head, Link::Empty)
        });
        self.head = Link::More(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        let result;
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => result = None
            Link::More(node) => {
                result = Some(node.elem);
                self.head = node.next;
            }
        };
    }
}

fn main() {
}
