use std::{env::temp_dir, fmt};

const N: usize = 30;
#[derive(Debug, Clone)]
struct Node {
    character: u8,
    terminal: bool,
    next: Vec<Option<Node>>,
}

#[derive(Debug, Clone)]
pub struct SuffixTree {
    size: usize,
    root: Node,
}

impl Node {
    fn new(character: u8) -> Self {
        let mut temp = Node {
            character,
            terminal: false,
            next: vec![],
        };
        for _ in 0..N {
            temp.next.push(None);
        }
        temp
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut temp_array = Vec::new();
        for i in 0..N {
            if let Some(node) = &self.next[i] {
                //println!("{}", &node);
                temp_array.push(format!("{}", &node));
            }
        }
        write!(f, "{} : {:#?}", self.character as char, temp_array)
    }
}

impl fmt::Display for SuffixTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Size : {},\n {}", self.size, self.root)
    }
}

impl SuffixTree {
    pub fn new() -> Self {
        SuffixTree {
            size: 0,
            root: Node::new(0 as u8),
        }
    }
    pub fn add_word(&mut self, word: Vec<u8>) {
        let n = word.len();
        let mut current = &mut self.root;
        for i in 0..n {
            if let None = current.next[word[i] as usize - 97] {
                let new = Node::new(word[i]);
                current.next[word[i] as usize - 97] = Some(new);
            }
            current = current.next[word[i] as usize - 97].as_mut().unwrap();
        }
        current.terminal = true;
    }
    pub fn delete_word(&mut self, word: Vec<u8>) {
        let n = word.len();
        let mut current = &mut self.root;
        for i in 0..n {
            if let None = current.next[word[i] as usize - 97] {
                return;
            }
            current = current.next[word[i] as usize - 97].as_mut().unwrap();
        }
        current.terminal = false;
    }
    pub fn contains_word(&self, word: Vec<u8>) -> bool {
        let n = word.len();
        let mut current = &self.root;
        for i in 0..n {
            if let None = current.next[word[i] as usize - 97] {
                return false;
            }
            current = current.next[word[i] as usize - 97].as_ref().unwrap();
        }
        current.terminal
    }
}
