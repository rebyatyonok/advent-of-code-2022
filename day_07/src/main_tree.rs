use std::fs;

#[derive(Debug)]
enum NodeType {
    File,
    Directory,
}

#[derive(Debug)]
struct Node<'a> {
    node_type: NodeType,
    size: Option<usize>,
    children: Vec<Node<'a>>,
    name: &'a str,
    parent: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    fn add_child(&'a mut self, child: Node<'a>) {
        let contains = self.children.iter().find(|node| node.name == child.name);

        match contains {
            Some(_) => (),
            None => {
                self.children.push(child);
            }
        };
    }

    fn find_child(&'a self, name: &str) -> &mut Node {
        let mut child = self.children.iter().find(|node| node.name == name);

        match child.as_mut() {
            Some(node) => node,
            None => panic!("No such child"),
        }
    }
}

fn construct_fs(file: &String) -> Node {
    let mut root = Node {
        node_type: NodeType::Directory,
        size: None,
        children: Vec::new(),
        name: "/",
        parent: None,
    };
    let mut current_dir = &mut root;

    for line in file.lines() {
        let mut tokens = line.split_whitespace();
        let first = tokens.next().expect("Line ended while first!");
        let second = tokens.next().expect("Something");

        match (first, second) {
            ("$", "cd") => {
                match tokens.next() {
                    Some("/") => (),
                    Some("..") => {
                        current_dir = match current_dir.parent.as_mut() {
                            Some(val) => &mut *val,
                            None => panic!("No parent!"),
                        };
                    }
                    Some(dirname) => {
                        let mut child = current_dir.find_child(dirname);
                        current_dir = &mut child;
                    }
                    None => panic!("Unknown direction to move"),
                };
            }
            ("$", "ls") => (),
            ("dir", dirname) => {
                // let new_dir = Node {
                //     node_type: NodeType::Directory,
                //     size: None,
                //     children: Vec::new(),
                //     name: dirname,
                //     parent: Box::new(None),
                // };
                // current_dir.add_child(new_dir);
            }
            (size, filename) => {}
        }
    }

    root
}

fn main() {
    let file = fs::read_to_string("day_07/src/input.txt").unwrap();
}
