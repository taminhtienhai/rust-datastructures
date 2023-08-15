use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Default)]
pub struct Node {
    key: isize,
    value: isize,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}

pub struct LRUCache<const N: usize> {
    cache: HashMap<isize, Rc<RefCell<Node>>>,
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
    cap: usize,
}

impl From<Node> for Rc<RefCell<Node>> {
    fn from(value: Node) -> Self {
        Rc::new(RefCell::new(value))
    }
}

impl Node {
    pub fn new(key: isize, value: isize) -> Self {
        Self {
            key,
            value,
            next: None,
            prev: None,
        }
    }

    pub fn remove(&self) {
        match (self.prev.as_ref(), self.next.as_ref()) {
            (Some(prev), Some(next)) => {
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev.clone());
            },
            _ => panic!("head and tail should never `None`"),
        };
    }
}

impl<const N: usize> LRUCache<N> {
    pub fn new() -> Self {
        let head: Rc<RefCell<Node>> = Node::new(-1, -1).into();
        let tail: Rc<RefCell<Node>> = Node::new(-1, -1).into();

        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(head.clone());

        let s = Self {
            head,
            tail,
            cache: HashMap::with_capacity(N),
            cap: N,
        };

        s
    }

    pub fn append_head(&mut self, node: Rc<RefCell<Node>>) {
        let after_node = self.head.borrow().next.clone();
        after_node.as_ref().map(|n| {
            n.borrow_mut().prev = Some(node.clone());
        });
        self.head.borrow_mut().next = Some(node.clone());
        node.borrow_mut().prev = Some(self.head.clone());
        node.borrow_mut().next = after_node.clone();
    }

    pub fn insert(&mut self, key: isize, value: isize) {
        let new_node: Rc<RefCell<Node>> = Node::new(key, value).into();
        if self.cache.contains_key(&key) {
            match self.cache.get(&key) {
                Some(existed) => {
                    existed.borrow().remove();
                    let delete_key = existed.borrow().key;
                    self.cache.remove(&delete_key);
                },
                _ => panic!("Must have the key"),
            };
            self.cache.insert(key, new_node.clone());
            self.append_head(new_node);
            return;
        }
        if self.cache.len() >= self.cap {
            let last_node = self.tail.borrow().prev.clone();
            match last_node {
                Some(least) => {
                    least.borrow().remove();
                    let delete_key = least.borrow().key;
                    self.cache.remove(&delete_key);
                },
                _ => panic!("The least item should exist"),
            };
        }
        self.cache.insert(key, new_node.clone());
        self.append_head(new_node);
    }

    pub fn get(&mut self, key: isize) -> isize {
        if let Some(node) = self.cache.get(&key) {
            let value = node.borrow().value;
            node.borrow().remove();
            self.append_head(node.clone());
            return value;
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::LRUCache;

    #[test]
    fn simple_use_case() {
        let mut lru = LRUCache::<2>::new();

        lru.insert(1, 1);
        lru.insert(2, 2);
        lru.insert(3, 3);

        assert_eq!(-1, lru.get(1));
        assert_eq!(2, lru.get(2));
        assert_eq!(3, lru.get(3));
    }

    #[test]
    fn duplicate_keys() {
        let mut lru = LRUCache::<2>::new();

        lru.insert(1, 1);
        lru.insert(3, 3);
        lru.insert(1, 2);
        lru.insert(4, 4);

        assert_eq!(2, lru.get(1));
        assert_eq!(-1, lru.get(3));
        assert_eq!(4, lru.get(4));
    }
}
