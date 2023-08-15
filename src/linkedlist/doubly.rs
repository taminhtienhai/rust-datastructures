use std::{cell::RefCell, rc::{Rc, Weak}};

type NodePtr<T> = Rc<RefCell<Node<T>>>;
type NodeWeakPtr<T> = Weak<RefCell<Node<T>>>;

pub struct Node<T> {
    pub value: T,
    pub next: Option<NodePtr<T>>,
    pub prev: Option<NodePtr<T>>,
}

pub struct NodeIterMut<'a, T>(&'a mut DoublyLinkedList<T>);

pub struct DoublyLinkedList<T> {
    pub head: Option<NodePtr<T>>,
    pub tail: Option<NodePtr<T>>,
}

impl <T> From<Node<T>> for NodePtr<T> {
    fn from(value: Node<T>) -> Self {
        Rc::new(RefCell::new(value))
    }
}

impl <T> Node<T> {
    fn new(item: T) -> Self {
        Self {
            value: item,
            next: None,
            prev: None,
        }
    }
}

impl <'a, T: Copy> Iterator for NodeIterMut<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl <T: Copy> DoublyLinkedList<T> {
    fn new() -> Self {
        Self { head: None, tail: None }
    }

    fn push_back(&mut self, item: T) {
        let mut node = Node::new(item);

        match self.tail.take() {
            None => {
                self.head = Some(node.into());
                self.tail = self.head.clone();
            }
            Some(cur_tail) => {
                node.prev = Some(cur_tail.clone());
                cur_tail.borrow_mut().next = Some(node.into());
                self.tail = cur_tail.borrow().next.clone();
            },
        }
    }

    fn push_front(&mut self, item: T) {
        let mut node = Node::new(item);

        match self.head.take() {
            None => {
                self.head = Some(node.into());
                self.tail = self.head.clone();
            },
            Some(cur_head) => {
                node.next = Some(cur_head.clone());
                cur_head.borrow_mut().prev = Some(node.into());
                self.head = cur_head.borrow().prev.clone();
            }
        }
    }
    
    fn pop_back(&mut self) -> Option<T> {
        match self.tail.take() {
            None => None,
            Some(cur_tail) => {
                let value = cur_tail.borrow().value;
                cur_tail.borrow_mut().prev.as_mut().map(|item| {
                    item.borrow_mut().next = None;
                    self.tail = Some(item.clone());
                });
                cur_tail.borrow_mut().prev = None;
                Some(value)
            }
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(cur_head) => {
                let value = cur_head.borrow().value;
                cur_head.borrow_mut().next.as_mut().map(|item| {
                    item.borrow_mut().prev = None;
                    self.head = Some(item.clone());
                });
                Some(value)
            }
        }
    }


    pub fn iter_mut(&mut self) -> NodeIterMut<'_, T> {
        NodeIterMut(self)
    }
}


#[cfg(test)]
mod tests {
    use super::DoublyLinkedList;


    #[test]
    fn list_push_items() {
        let mut list = DoublyLinkedList::<usize>::new();
        list.push_back(10);
        list.push_back(11);
        list.push_back(12);
        list.push_back(13);

        assert_eq!(list.pop_back(), Some(13));
        assert_eq!(list.pop_back(), Some(12));
        assert_eq!(list.pop_back(), Some(11));
        assert_eq!(list.pop_back(), Some(10));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn list_pop_front_items() {
        let mut list = DoublyLinkedList::<usize>::new();
        list.push_back(10);
        list.push_back(11);
        list.push_back(12);
        list.push_back(13);

        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.pop_front(), Some(11));
        assert_eq!(list.pop_front(), Some(12));
        assert_eq!(list.pop_front(), Some(13));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn list_push_front_items() {
        let mut list = DoublyLinkedList::<usize>::new();
        list.push_front(10);
        list.push_front(11);
        list.push_front(12);
        list.push_front(13);

        assert_eq!(list.pop_back(), Some(10));
        assert_eq!(list.pop_back(), Some(11));
        assert_eq!(list.pop_back(), Some(12));
        assert_eq!(list.pop_back(), Some(13));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn iterate_list() {
        let mut list = DoublyLinkedList::<usize>::new();
        list.push_back(10);
        list.push_back(11);
        list.push_back(12);
        list.push_back(13);

        for item in list.iter_mut() {
            println!("{item:?}");
        }
    }
}