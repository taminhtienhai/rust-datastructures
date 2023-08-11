use std::ops::DerefMut;

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct LinearLinkedList<T> {
    pub root: Node<T>,
}

pub trait LinkedFunc<T> {
    fn insert(&mut self, item: T);
}

impl <T> LinkedFunc<T> for LinearLinkedList<T> {

    fn insert(&mut self, item: T) {
        let mut pointer: &mut Node<T> = &mut self.root;

        while let Some(ref mut p) = pointer.next {
            // pointer = p; /// bellow is the expansion of this line
            pointer = p.deref_mut() as &mut Node<T>;
        }

        pointer.next = Some(Box::new(Node { value: item, next: None }));
    }
}