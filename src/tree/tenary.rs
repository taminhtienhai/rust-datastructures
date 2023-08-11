use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

type Parent<T> = Weak<RefCell<Node<T>>>;
type Leaf<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub parent: Option<Parent<T>>,
    pub left: Option<Leaf<T>>,
    pub middle: Option<Leaf<T>>,
    pub right: Option<Leaf<T>>,
}

pub struct TenaryTree<T> {
    root: Option<Leaf<T>>,
}

trait PropTaker<T> {
    fn middle_val(&self) -> T;
    fn left_val(&self) -> T;
    fn right_val(&self) -> T;
}

impl<T> From<Node<T>> for Leaf<T> {
    fn from(value: Node<T>) -> Self {
        Rc::new(RefCell::new(value))
    }
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            parent: None,
            left: None,
            middle: None,
            right: None,
        }
    }
}

impl <T: Copy> PropTaker<T> for Leaf<T> {
    fn middle_val(&self) -> T {
        self.borrow().middle.clone().unwrap().borrow().value
    }

    fn left_val(&self) -> T {
        self.borrow().left.clone().unwrap().borrow().value
    }

    fn right_val(&self) -> T {
        self.borrow().right.clone().unwrap().borrow().value
    }
}

impl TenaryTree<char> {
    fn new() -> Self {
        Self {
            root: None
        }
    }

    fn insert(&mut self, word: &str) {
        let mut cursor: Leaf<char>;
        let mut chars = word.chars().into_iter();

        if self.root.is_none() {
            let next_char = chars.next();
            match next_char {
                Some(c) => {
                    self.root = Some(Node::new(c).into());
                    cursor = self.root.clone().unwrap();
                }
                _ => {
                    return;
                }
            };

            while let Some(c) = chars.next() {
                cursor.borrow_mut().middle = Some(Node::new(c).into());
                let new_cursor = cursor.borrow().middle.clone().unwrap();
                cursor = new_cursor;
            }
            return;
        }

        cursor = self.root.clone().unwrap();

        // find the root
        while let Some(c) = chars.next() {
            if cursor.borrow().value == c {
                if cursor.borrow().middle.is_none() {
                    cursor.borrow_mut().middle = Some(Node::new(c).into());
                    let new_cursor = cursor.borrow().middle.clone().unwrap();
                    cursor = new_cursor;
                    break;
                }
                let new_cursor = cursor.borrow().middle.clone().unwrap();
                cursor = new_cursor;
                continue;
            }
            if c > cursor.borrow().value {
                if cursor.borrow().right.is_none() {
                    cursor.borrow_mut().right = Some(Node::new(c).into());
                    let new_cursor = cursor.borrow().right.clone().unwrap();
                    cursor = new_cursor;
                    break;
                }
                let new_cursor = cursor.borrow().right.clone().unwrap();
                cursor = new_cursor;
                continue;
            }
            if c < cursor.borrow().value {
                if cursor.borrow().left.is_none() {
                    cursor.borrow_mut().left = Some(Node::new(c).into());
                    let new_cursor = cursor.borrow().left.clone().unwrap();
                    cursor = new_cursor;
                    break;
                }
                let new_cursor = cursor.borrow().left.clone().unwrap();
                cursor = new_cursor;
                continue;
            }
        }

        while let Some(c) = chars.next() {
            if cursor.borrow().middle.is_none() {
                cursor.borrow_mut().middle = Some(Node::new(c).into());
                let new_cursor = cursor.borrow().middle.clone().unwrap();
                cursor = new_cursor;
                continue;
            }

            if cursor.borrow().middle.is_some() && cursor.middle_val() == c {
                let new_cursor = cursor.borrow().middle.clone().unwrap();
                cursor = new_cursor;
                continue;
            }

            if cursor.borrow().right.is_some() && cursor.right_val() == c {
                let new_cursor = cursor.borrow().right.clone().unwrap();
                cursor = new_cursor;
                continue;
            }

            if cursor.borrow().left.is_some() && cursor.left_val() == c {
                let new_cursor = cursor.borrow().left.clone().unwrap();
                cursor = new_cursor;
                continue;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TenaryTree;


    #[test]
    fn insert_nodes() {
        let mut tree = TenaryTree::new();

        tree.insert("apple");
        tree.insert("peal");
        tree.insert("lemon");


        println!("{:?}", tree.root);
    }
}