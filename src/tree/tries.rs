use std::{collections::HashMap, hash::Hash, marker::PhantomData};

#[derive(PartialEq, Eq, Clone)]
pub struct Node<T: Hash + Eq> {
    pub value: T,
    pub childs: HashMap<T, Node<T>>,
}

pub struct Tries<T: Hash + Eq, Iter> {
    pub childs: HashMap<T, Node<T>>,
    _phantom: PhantomData<Iter>,
}

impl<T: Hash + Eq> Hash for Node<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T: Hash + Eq> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            childs: HashMap::default(),
        }
    }
}

impl<T: Hash + Eq + Clone, V: IntoIterator<Item = T>> Tries<T, V> {
    fn new() -> Self {
        Self {
            childs: HashMap::new(),
            _phantom: PhantomData::default(),
        }
    }

    fn insert(&mut self, value: V) {
        let mut cursor: &mut Node<T>;
        let mut items = value.into_iter();
        let first_item = items.next();


        match first_item {
            Some(item) => {
                // let n = Node::new(item);
                match self.childs.get_mut(&item) {
                    Some(node) => {
                        cursor = node;
                    }
                    _ => {
                        self.childs.insert(item.clone(), Node::new(item.clone()));
                        cursor = self.childs.get_mut(&item).unwrap();
                    }
                }
            }
            _ => {
                panic!("At least one item");
            }
        }

        while let Some(item) = items.next() {
            if cursor.childs.contains_key(&item) {
                cursor = cursor.childs.get_mut(&item).unwrap();
            } else {
                cursor.childs.insert(item.clone(), Node::new(item.clone()));
                cursor = cursor.childs.get_mut(&item).unwrap();
            }
        }
    }

    fn traverse(&self, node: &Node<T>, collector: &mut Vec<T>) -> Vec<Vec<T>> {
        let mut rs = Vec::<Vec<T>>::new();

        for (key, value) in &node.childs {
            let mut new_collector = collector.clone();
            new_collector.push(key.clone());
            let o_rs = self.traverse(value, &mut new_collector);
            rs.push(new_collector);
            rs.extend(o_rs);   
        }

        rs
    } 

    fn auto_complete(&self, word: V) -> Vec<Vec<T>> {
        let mut words = word.into_iter();
        let first_word = words.next();
        let mut cursor: &Node<T>;

        match first_word {
            Some(w) => {
                if self.childs.contains_key(&w) {
                    cursor = self.childs.get(&w).unwrap();
                } else {
                    return Vec::new();
                }
            },
            _ => {
                return Vec::new();
            }
        }

        while let Some(w) = words.next() {
            if cursor.childs.contains_key(&w) {
                cursor = cursor.childs.get(&w).unwrap();
            } else {
                return Vec::new();
            }
        }

        let mut collector = Vec::<T>::new();

        self.traverse(cursor, &mut collector)
    }
}

#[cfg(test)]
mod tests {
    use super::Tries;

    #[test]
    fn insert_nodes() {
        let mut tries = Tries::new();

        tries.insert("hello".chars());
        tries.insert("help".chars());
        tries.insert("world".chars());


        let words = tries.auto_complete("he".chars());

        println!("{words:?}");
    }
}
