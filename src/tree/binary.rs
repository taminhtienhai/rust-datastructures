use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Debug,
    rc::{Rc, Weak},
};

pub type LeafPtr<T> = Option<Rc<RefCell<Node<T>>>>;
pub type ParentPtr<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Debug, Clone)]
pub struct Node<V> {
    pub value: V,

    pub parent: ParentPtr<V>,
    pub left: LeafPtr<V>,
    pub right: LeafPtr<V>,
}

pub struct BinaryTree<V> {
    root: Rc<RefCell<Node<V>>>,
    length: usize,
}

impl<V> From<Node<V>> for LeafPtr<V> {
    fn from(value: Node<V>) -> Self {
        Some(Rc::new(RefCell::new(value)))
    }
}

impl<V> From<Node<V>> for Rc<RefCell<Node<V>>> {
    fn from(value: Node<V>) -> Self {
        Rc::new(RefCell::new(value))
    }
}

impl<V: PartialEq> Node<V> {
    fn new(value: V) -> Self {
        Self {
            value,
            parent: None,
            left: None,
            right: None,
        }
    }

    fn with_parent(mut self, parent: &Rc<RefCell<Node<V>>>) -> Self {
        self.parent = Some(Rc::downgrade(&parent));
        self
    }

    fn equals(&self, other: V) -> bool {
        return self.value == other;
    }
}

impl<V: PartialEq + PartialOrd + Copy + Debug> BinaryTree<V> {
    fn new(value: V) -> Self {
        Self {
            root: Rc::new(RefCell::new(Node::new(value))),
            length: 1,
        }
    }

    fn insert(&mut self, value: V) {
        let mut cursor = self.root.clone();

        loop {
            if value < cursor.borrow().value {
                if cursor.borrow().left.is_none() {
                    let node = Node::new(value).with_parent(&cursor);
                    cursor.borrow_mut().left.replace(node.into());
                    break;
                }

                let new_cursor = cursor.borrow_mut().left.clone().unwrap();
                cursor = new_cursor;
            }

            if value > cursor.borrow().value {
                if cursor.borrow().right.is_none() {
                    let node = Node::new(value).with_parent(&cursor);
                    cursor.borrow_mut().right.replace(node.into());
                    break;
                }

                let new_cursor = cursor.borrow_mut().right.clone().unwrap();
                cursor = new_cursor;
            }
        }

        self.length += 1;
    }

    fn delete(&mut self, value: V) -> bool {
        let detach_node = |target: Rc<RefCell<Node<V>>>| {
            let target_value = target.borrow().value;
            if let Some(parent) = target.borrow().parent.clone() {
                let p = parent.upgrade().unwrap();
                if p.borrow()
                    .left
                    .clone()
                    .is_some_and(|l| l.borrow().value == target_value)
                {
                    p.borrow_mut().left = None;
                } else {
                    p.borrow_mut().right = None;
                }
            }
        };
        if let Some(target) = self.find(value) {
            // if the node have none childs
            if target.borrow().left.is_none() & target.borrow().right.is_none() {
                detach_node(target.clone());
                return true;
            }

            let left_clone = target.borrow().left.clone();
            if let Some(left_node) = left_clone {
                if let Some(max_node) = self.max(left_node.clone()) {
                    let mut new_node = Node::new(max_node.borrow().value);
                    new_node.left = target.borrow().left.clone();
                    new_node.right = target.borrow().right.clone();
                    if max_node.borrow().equals(left_node.borrow().value) {
                        new_node = Node::new(left_node.borrow().value);
                        new_node.left = left_node.borrow().left.clone();
                        new_node.right = left_node.borrow().right.clone();


                        target.replace(new_node);
                        return true;
                    }

                    target.replace(new_node);
                    detach_node(max_node.clone());
                }

                return true;
            }

            let right_clone = target.borrow().right.clone();
            if let Some(right_node) = right_clone {
                if let Some(min_node) = self.min(right_node.clone()) {
                    let mut new_node = Node::new(min_node.borrow().value);
                    new_node.left = target.borrow().left.clone();
                    new_node.right = target.borrow().right.clone();

                    if min_node.borrow().equals(right_node.borrow().value) {
                        new_node = Node::new(right_node.borrow().value);
                        new_node.left = right_node.borrow().left.clone();
                        new_node.right = right_node.borrow().right.clone();

                        target.replace(new_node);
                        return true;
                    }
                    target.replace(new_node);
                    detach_node(min_node.clone());
                }

                return true;
            }
        }

        return false;
    }

    fn min(&self, node: Rc<RefCell<Node<V>>>) -> LeafPtr<V> {
        let mut min_node = node.clone();

        loop {
            if min_node.borrow().left.is_none() {
                break;
            }

            if min_node
                .borrow()
                .left
                .as_ref()
                .is_some_and(|n| n.borrow().value > min_node.borrow().value)
            {
                let new_node = min_node.borrow().left.clone();
                min_node = new_node.unwrap();
            }
        }

        Some(min_node)
    }

    fn max(&self, node: Rc<RefCell<Node<V>>>) -> LeafPtr<V> {
        let mut max_node = node.clone();

        loop {
            if max_node.borrow().right.is_none() {
                break;
            }

            if max_node
                .borrow()
                .right
                .as_ref()
                .is_some_and(|n| n.borrow().value > max_node.borrow().value)
            {
                let new_node = max_node.borrow().right.clone();
                max_node = new_node.unwrap();
            }
        }

        Some(max_node)
    }

    fn find(&self, value: V) -> LeafPtr<V> {
        let mut cursor = self.root.clone();
        loop {
            if cursor.borrow().value == value {
                return Some(cursor);
            }

            if value < cursor.borrow().value {
                if cursor.borrow().left.is_some() {
                    let new_cursor = cursor.borrow().left.clone().unwrap();
                    cursor = new_cursor
                } else {
                    break;
                }
            }

            if value > cursor.borrow().value {
                if cursor.borrow().right.is_some() {
                    let new_cursor = cursor.borrow().right.clone().unwrap();
                    cursor = new_cursor;
                } else {
                    break;
                }
            }
        }
        return None;
    }

    fn search(&self, value: V) -> bool {
        let mut cursor = self.root.clone();
        loop {
            if cursor.borrow().value == value {
                return true;
            }

            if value < cursor.borrow().value {
                if cursor.borrow().left.is_some() {
                    let new_cursor = cursor.borrow().left.clone().unwrap();
                    cursor = new_cursor
                } else {
                    break;
                }
            }

            if value > cursor.borrow().value {
                if cursor.borrow().right.is_some() {
                    let new_cursor = cursor.borrow().right.clone().unwrap();
                    cursor = new_cursor;
                } else {
                    break;
                }
            }
        }
        return false;
    }

    fn traverse(&mut self, node: &Ref<'_, Node<V>>, collector: &mut Vec<V>) {
        if let Some(ref l) = node.left {
            self.traverse(&l.clone().borrow(), collector);
        }

        collector.push(node.value);

        if let Some(ref r) = node.right {
            self.traverse(&r.clone().borrow(), collector);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BinaryTree;

    #[test]
    fn new_tree() {
        let mut tree = BinaryTree::new(10);

        tree.insert(9);
        tree.insert(11);
        tree.insert(12);
        tree.insert(18);
        tree.insert(4);
        tree.insert(3);
        tree.insert(5);

        assert_eq!(true, tree.search(10));
        assert_eq!(true, tree.search(9));
        assert_eq!(true, tree.search(11));
        assert_eq!(true, tree.search(12));
        assert_eq!(true, tree.search(18));

        assert_eq!(false, tree.search(8));
        assert_eq!(false, tree.search(20));
    }

    #[test]
    fn delete_tree_node() {
        let mut tree = BinaryTree::new(10);

        tree.insert(9);
        tree.insert(11);
        tree.insert(12);
        tree.insert(18);
        tree.insert(4);
        tree.insert(3);
        tree.insert(5);

        tree.delete(11);

        assert_eq!(true, tree.search(9));
        assert_eq!(true, tree.search(10));
        assert_eq!(true, tree.search(12));
        assert_eq!(true, tree.search(18));
        assert_eq!(true, tree.search(4));
        assert_eq!(true, tree.search(3));
        assert_eq!(true, tree.search(5));

        assert_eq!(false, tree.search(11));
    }

    #[test]
    fn traverse_tree() {
        let mut tree = BinaryTree::new(10);

        tree.insert(9);
        tree.insert(11);
        tree.insert(12);
        tree.insert(18);
        tree.insert(4);
        tree.insert(3);
        tree.insert(5);


        let mut collector = Vec::new();
        tree.traverse(&tree.root.clone().borrow(), &mut collector);

        assert_eq!(&[3,4,5,9,10,11,12,18], collector.as_slice());
    }
}
