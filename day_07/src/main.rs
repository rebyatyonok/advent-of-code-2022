#[derive(Debug, PartialEq, Eq, Hash)]
enum Node {
    File(usize),
    Directory(usize),
}

enum Command<'a> {
    Cd(&'a str),
    Ls,
    AddDir(&'a str),
    AddFile(usize, &'a str),
}

#[derive(Debug)]
struct Directory {
    children: std::collections::HashMap<String, Node>,
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
            children: Default::default(),
            parent: 0,
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
            name => self.directories[self.current]
                .children
                .get(name)
                .and_then(|node| match node {
                    Node::File { .. } => panic!("Trying to cd to file with name {}", name),
                    Node::Directory(dir) => Some(*dir),
                })
                .unwrap(),
            // name => match self.directories.iter().position(|e| e.name == name) {
            //     Some(index) => index,
            //     None => panic!("Can't find a dir in {:?} to {}", self.directories, name),
            // },
        }
    }

    fn add_file(&mut self, size: usize, name: &str) {
        self.directories[self.current]
            .children
            .entry(name.to_string())
            .or_insert_with(|| Node::File(size));
    }

    fn add_directory(&mut self, name: &str) {
        let directory = Directory {
            children: Default::default(),
            parent: self.current,
        };
        self.directories.push(directory);
        let index = self.directories.len() - 1;

        self.directories[self.current]
            .children
            .entry(name.to_string())
            .or_insert_with(|| Node::Directory(index));
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
        (size, name) => {
            let size = size.parse::<usize>().expect("Expected parsable string");
            return Command::AddFile(size, name);
        }
    }
}

fn get_dir_size(dir: &Directory, fs: &FileSystem) -> usize {
    dir.children
        .iter()
        .map(|(_, child)| match child {
            &Node::File(size) => size,
            &Node::Directory(index) => get_dir_size(&fs.directories[index], fs),
        })
        .sum()
}

fn get_dir_sizes(fs: &FileSystem) -> Vec<usize> {
    fs.directories
        .iter()
        .enumerate()
        .rev()
        .map(|(_, dir)| get_dir_size(dir, fs))
        .collect()
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
            Command::AddFile(size, name) => fs.add_file(size, name),
        }
    }

    let limit = 100000 as usize;

    let first_task: usize = get_dir_sizes(&fs).into_iter().filter(|e| e < &limit).sum();

    println!("{:#?}", first_task);
}
