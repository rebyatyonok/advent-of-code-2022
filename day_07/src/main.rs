use std::fs::File;

#[derive(Debug)]
enum Node {
    File(usize),
    Directory(usize),
}

enum Command<'a> {
    Cd(&'a str),
    Ls,
    AddDir(&'a str),
    AddFile(usize),
}

#[derive(Debug)]
struct Directory {
    name: String,
    children: Vec<Node>,
    parent: usize,
}

#[derive(Debug)]
struct FileSystem {
    directories: Vec<Directory>,
    current: usize,
}

impl FileSystem {
    fn new() -> FileSystem {
        let root = Directory {
            children: vec![],
            parent: 0,
            name: String::from("root"),
        };

        FileSystem {
            directories: vec![root],
            current: 0,
        }
    }

    fn cd(&mut self, to: &str) {
        self.current = match to {
            "/" => 0,
            ".." => self.directories[self.current].parent,
            name => match self.directories.iter().position(|e| e.name == name) {
                Some(index) => index,
                None => panic!("Can't find a dir in {:?} to {}", self.directories, name),
            },
        }
    }

    fn add_file(&mut self, size: usize) {
        self.directories[self.current]
            .children
            .push(Node::File(size));
    }

    fn add_directory(&mut self, name: &str) {
        let directory = Directory {
            name: name.to_string(),
            children: vec![],
            parent: self.current,
        };
        let len = self.directories.len();
        self.directories.push(directory);
        self.directories[self.current]
            .children
            .push(Node::Directory(len + 1));
    }
}

fn parse_line(line: &str) -> Command {
    let mut tokens = line.split_whitespace();
    let first = tokens.next().expect("Line ended while first!");
    let second = tokens.next().expect("Something");

    match (first, second) {
        ("$", "cd") => {
            let direction = tokens.next().expect("Need direction to cd");
            return Command::Cd(direction);
        }
        ("$", "ls") => Command::Ls,
        ("dir", dirname) => Command::AddDir(dirname),
        (size, _) => {
            let size = size.parse::<usize>().expect("Expected parsable string");
            return Command::AddFile(size);
        }
    }
}

fn get_dir_sizes(fs: &FileSystem) -> usize {
    let mut sizes = Vec::with_capacity(fs.directories.len());
    sizes.resize(fs.directories.len(), 0);

    fs.directories
        .iter()
        .enumerate()
        .rev()
        .for_each(|(i, dir)| {
            let dir_size = dir
                .children
                .iter()
                .map(|child| match child {
                    &Node::File(size) => size,
                    &Node::Directory(index) => sizes[index],
                })
                .sum();

            sizes[i] = dir_size;
        });

    sizes.iter().sum()
}

fn main() {
    let file = std::fs::read_to_string("day_07/src/input.txt").unwrap();
    let mut fs = FileSystem::new();

    for line in file.lines() {
        let command = parse_line(line);

        match command {
            Command::Cd(path) => fs.cd(path),
            Command::Ls => (),
            Command::AddDir(dirname) => fs.add_directory(dirname),
            Command::AddFile(size) => fs.add_file(size),
        }
    }

    println!("{:#?}", get_dir_sizes(&fs));
}
