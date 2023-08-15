use std::{cell::{RefCell, Ref}, rc::{Rc, Weak}};

type NodePtr<T> = Rc<TreeNode<T>>;
type ParentPtr<T> = Weak<RefCell<TreeNode<T>>>;

trait Iterable {
    type Item<'collection>
    where
        Self: 'collection;

    type Iterator<'collection>: Iterator<Item = Self::Item<'collection>>
    where
        Self: 'collection;

    fn iter<'c>(&'c self) -> Self::Iterator<'c>;
}

trait LendingIterator {
    type Item<'a>
        where Self: 'a;

    fn next<'c>(&'c mut self) -> Option<Self::Item<'c>>;
}

pub struct NodeIter<It>(Rc<TreeNode<It>>, It);

pub struct TreeNode<T> {
    value: T,
    left: Option<NodePtr<T>>,
    right: Option<NodePtr<T>>,
}

pub struct Tree<T> {
    root: Option<NodePtr<T>>
}

// impl <T> From<TreeNode<T>> for NodePtr<T> {
//     fn from(value: TreeNode<T>) -> Self {
//         Rc::new(RefCell::new(value))
//     }
// }

impl <T> TreeNode<T> {
    pub fn new(value: T) -> Self { Self { value, left: None, right: None, } }
}

impl <T: Clone> Tree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn iter(&self, item: T) -> NodeIter<T> {
        match &self.root {
            Some(it) => NodeIter(it.clone(), item),
            _ => panic!("")
        }
    }
}

impl <'a, It: PartialOrd + Clone> LendingIterator for NodeIter<It> {
    type Item<'c> = &'c It
        where Self: 'c;

    fn next<'c>(&'c mut self) -> Option<Self::Item<'c>> {
        match (self.0.left.as_ref(), self.0.right.as_ref()) {
            (Some(left_ref), _) if self.1 < left_ref.value => {
                self.0 = left_ref.clone();
                Some(&self.1)
            },
            (_, Some(right_ref)) if self.1 > right_ref.value => {
                self.0 = right_ref.clone();
                Some(&self.1)
            },
            _ => None,
        }
    }
}