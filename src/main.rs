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
        match &self.nodes[char_value] {
            Some(_) => (),
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
}
impl Node {
    fn new(char: char, terminal: bool) -> Self {
        Self {
            value: char,
            children: NodeList::new(),
            terminal,
        }
    }
    fn words_from(&self) -> Vec<String> {
        //descend tree
        //when finding a terminal node, push that node to the list
        unimplemented!()
    }
}

fn main() {
    let mut root = NodeList::new();
    root.add_word("abc");
    println!("{:?}", root);
    root.add_word("abd");
    println!("{:?}", root);
}
