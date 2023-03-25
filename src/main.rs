use std::{path::Path, thread::current};

#[derive(Clone, Debug)]
struct NodeList {
    nodes: Vec<Option<Box<Node>>>,
}
#[derive(Clone, Debug)]
struct Node {
    value: char,
    children: NodeList,
    terminal: bool,
}
impl NodeList {
    fn new() -> Self {
        NodeList {
            nodes: vec![None; 26],
        }
    }
    fn add_char(&mut self, char: char, terminal: bool) {
        let char_value = (char as u8 - b'a') as usize;
        match &mut self.nodes[char_value] {
            Some(child) => {
                if child.terminal {
                    return;
                }
                child.terminal = terminal;
            }
            None => self.nodes[char_value] = Some(Box::new(Node::new(char, terminal))),
        }
    }
    fn add_word(&mut self, word: &str) {
        let mut current_list: &mut NodeList = self;
        let length = word.len();

        for (i, char) in word.chars().enumerate() {
            if i == length - 1 {
                current_list.add_char(char, true)
            } else {
                current_list.add_char(char, false);
                let letter_index = (char as u8 - b'a') as usize;

                match current_list.nodes[letter_index].as_mut() {
                    Some(list) => current_list = &mut list.children,
                    None => unreachable!(),
                }
            }
        }
    }
    fn get_all_words(&self) -> Vec<String> {
        let mut words: Vec<String> = Vec::new();

        if let Some(children) = self.existing_nodes() {
            for child in children {
                words.append(&mut child.words_from())
            }
        }
        words
    }
    fn existing_nodes(&self) -> Option<Vec<&Box<Node>>> {
        let children: Vec<&Box<Node>> = self
            .nodes
            .iter()
            .filter(|c| c.is_some())
            .map(|n| n.as_ref().unwrap())
            .collect();
        if children.len() == 0 {
            return None;
        }
        Some(children)
    }
    fn find_starts_with(&self, str: &str) -> Vec<String> {
        let mut current_index = self;
        for char in str.chars() {
            let value = (char as u8 - b'a') as usize;
            if let Some(current_node) = &current_index.nodes[value] {
                current_index = &current_node.children;
            } else {
                panic!()
            }
        }
        current_index
            .get_all_words()
            .iter()
            .map(|x| format!("{}{}", str, x))
            .collect()
    }
}
impl Node {
    fn new(char: char, terminal: bool) -> Self {
        Self {
            value: char,
            children: NodeList::new(),
            terminal,
        }
    }
    fn get_children(&self) -> Option<Vec<&Box<Node>>> {
        self.children.existing_nodes()
    }
    fn words_from(&self) -> Vec<String> {
        // push letter to stack
        let mut letter_stack = vec![];
        let mut word_list = vec![];

        fn helper<'a>(node: &'a Node, stack: &mut Vec<&'a Node>, word_list: &mut Vec<String>) {
            stack.push(node);
            if node.terminal {
                word_list.push(stack.iter().map(|x| x.value).collect());
            }
            if let Some(children) = node.get_children() {
                for child in children {
                    helper(child, stack, word_list);
                    stack.pop();
                }
            }
        }

        helper(self, &mut letter_stack, &mut word_list);

        word_list
    }
}

fn main() {
    let mut root = NodeList::new();
    // root.add_word("a");
    // root.add_word("an");
    // root.add_word("ant");
    // root.add_word("anti");
    // root.add_word("anagram");
    // root.add_word("basic");
    // root.add_word("basically");
    // println!("{:?}", root.get_all_words());
    let words = std::fs::read_to_string(Path::new("/usr/share/dict/american-english")).unwrap();
    for word in words.split("\n") {
        if word.contains(|c: char| !c.is_ascii_alphabetic()) {
            continue;
        }
        root.add_word(&word.to_ascii_lowercase());
    }
    println!("{:?} Words Added", root.get_all_words().len());
    println!("Search for:");
    let mut usr_input = String::new();
    std::io::stdin().read_line(&mut usr_input).unwrap();
    usr_input = usr_input.trim().to_string();
    println!(
        "Begin with {}: {:?}",
        &usr_input,
        root.find_starts_with(&usr_input)
    );
}

#[cfg(test)]
#[test]
fn test_get_nodes() {
    let mut root = NodeList::new();
    root.add_word("abc");
    assert_eq!(root.existing_nodes().unwrap().len(), 1);
    root.add_word("bc");
    assert_eq!(root.existing_nodes().unwrap().len(), 2);
}
