use std::{cmp::Ordering};


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    value: char,
    childs: Vec<Node>,
    is_end: bool,
}

pub struct Trie {
    root: Vec<Node>,
}

impl Node {
    fn new(value: char) -> Self {
        Self { value, childs: Vec::default(), is_end: false }
    }

    fn get_value(&self) -> char { self.value }

    fn printf(&self) {
        println!("Character {:?} -> {:?}", self.value, self.childs.iter().map(Node::get_value).collect::<Vec<_>>());
        for node in self.childs.iter() {
            node.printf();
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.value > other.value { Ordering::Greater }
        else if self.value < other.value { Ordering::Less }
        else { Ordering::Equal }
    }
}

impl Trie {
    fn new() -> Self {
        Self { root: Vec::default() }
    }

    fn insert(&mut self, word: &str) {
        let mut cursor: &mut Node;
        let mut items = word.chars();
        let first_char = items.next();

        self.root.sort();

        if let Ok(index) = self.root.binary_search(&Node::new(first_char.clone().unwrap())) {
            cursor = self.root.get_mut(index).unwrap();
        } else {
            self.root.push(Node::new(first_char.unwrap()));
            cursor = self.root.last_mut().unwrap();
        }


        while let Some(item) = items.next() {
            cursor.childs.sort();
            if let Ok(next_node) = cursor.childs.binary_search(&Node::new(item.clone())) {
                cursor = cursor.childs.get_mut(next_node).unwrap();
            } else {
                cursor.childs.push(Node::new(item));
                cursor = cursor.childs.last_mut().unwrap();
            }
        }

        cursor.is_end = true;
    }

    fn traverse_tree(&self, node: &Node, collector: &mut Vec<String>) -> Vec<String> {
        let mut inner_collector = Vec::new();
        collector.push(node.value.to_string());

        if node.childs.len() < 1 {
            return Vec::from_iter([collector.join("")]);
        }

        for n in node.childs.iter() {
            let result = self.traverse_tree(n, &mut Vec::from_iter([collector.join("")]));
            inner_collector.extend(result);
        }

        if node.is_end {
            inner_collector.push(collector.join(""));
        }

        return inner_collector;
    }

    fn auto_complete(&self, word: &str) -> Vec<String> {
        // let mut collector = word.to_string();
        let mut items = word.chars();
        let first_char = items.next();
        let mut cursor: &Node;

        if let Ok(index) = self.root.binary_search(&Node::new(first_char.clone().unwrap())) {
            cursor = self.root.get(index).unwrap();
        } else {
            return Vec::from_iter([word.to_string()]);
        }

        for item in items {
            if let Ok(index) = cursor.childs.binary_search(&Node::new(item.clone())) {
                cursor = cursor.childs.get(index).unwrap();
            }
        }

        let mut collector = Vec::<String>::new();

        if cursor.is_end {
            collector.push(word.to_string());
        }
        for item in cursor.childs.iter() {
            let items = self.traverse_tree(item, &mut Vec::<String>::from_iter([word.to_string()]));
            collector.extend(items);
        }

        collector
    }
}

#[cfg(test)]
mod tests {
    use super::{Trie, Node};

    #[test]
    fn binary_search() {
        let node1 = Node::new('h');
        let node2 = Node::new('c');
        let node3 = Node::new('a');

        let mut root = Vec::new();
        root.push(node1);
        root.push(node2);
        root.push(node3);

        root.sort();

        println!("{root:?}");
    }

    #[test]
    fn insert_nodes() {
        let mut trie = Trie::new();

        trie.insert("hello");
        trie.insert("help");
        trie.insert("hell");
        trie.insert("application");
        trie.insert("applications");
        trie.insert("applicationssss");
        trie.insert("apple");

        let words = trie.auto_complete("app");

        println!("{words:?}");
    }
}