mod linkedlist;
mod tree;

use linkedlist::linear::{self, Node, LinkedFunc};

fn main() {
    let mut lk = linear::LinearLinkedList { root: Node { value: "a", next: None } };

    lk.insert("b");
    lk.insert("c");


    println!("{:?}", lk.root);
}
