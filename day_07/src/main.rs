use std::collections::HashMap;

type EntriesType = HashMap<&'static str, DirInfo>;

#[derive(Debug, Default)]
struct DirInfo {
    size: i32,
    parent: &'static str,
}

struct FS {
    root: i32,
    children: EntriesType,
    current_name: &'static str,
}

impl FS {
    fn add_size(&mut self, size: &str) {
        let num_size = size.parse::<i32>().unwrap();
        self.get_current().size += num_size;
    }

    fn add_dir(&mut self, name: &'static str, size: i32) {
        let child = DirInfo {
            size,
            parent: self.current_name,
        };

        match self.children.insert(name, child) {
            Some(_) => println!("{} already inserted", &name),
            None => (),
        }
    }

    fn get_current(&mut self) -> &mut DirInfo {
        match self.children.get_mut(self.current_name) {
            Some(dir) => dir,
            None => panic!("Can't get current {}", self.current_name),
        }
    }

    fn go_to(&mut self, direction: &'static str) {
        match direction {
            ".." => {
                let current_size = self.get_current().size;
                self.current_name = self.get_current().parent;
                self.get_current().size += current_size;
            }
            dir_name => {
                self.current_name = dir_name;
            }
        }
    }
}

fn get_entries(file: String) -> EntriesType {
    let mut FS = FS {
        root: 0,
        children: HashMap::new(),
        current_name: "/",
    };

    for line in file.lines() {
        let mut tokens = line.split_whitespace();
        let first = tokens.next().unwrap();

        match first {
            "$" => {
                let command = tokens.next().unwrap();
                let arg = tokens.next();

                match (command, arg) {
                    ("cd", Some(arg)) => FS.go_to(arg),
                    // ("cd", Some("..")) => {
                    //     let current_dir = entries.get(&current_dir_name).unwrap();
                    //     current_dir_name = current_dir.parent.clone()
                    // }
                    // ("cd", Some(dir)) => current_dir_name = String::from(dir),
                    ("ls", _) => (),
                    _ => panic!("Unknown command"),
                }
            }
            "dir" => {
                let dirname = tokens.next().unwrap();
                FS.add_dir(dirname, 0)
            }
            size => FS.add_size(size),
        }
    }

    FS.children
}

fn first_task(entries: &EntriesType) -> i32 {
    entries
        .values()
        .filter(|e| e.size <= 100000)
        .map(|e| e.size)
        .sum()
}

fn main() {
    let file = std::fs::read_to_string("day_07/src/input.txt").unwrap();
    let entries = get_entries(file);

    println!("{}", first_task(&entries));
}
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn correct_dirs_count() {
        let file = std::fs::read_to_string("src/test_input.txt").unwrap();
        let entries = get_entries(file);

        println!("{:?}", entries);
        assert_eq!(entries.values().len(), 4);
    }

    #[test]
    fn correct_output() {
        let file = std::fs::read_to_string("src/test_input.txt").unwrap();
        let entries = get_entries(file);

        assert_eq!(first_task(&entries), 95437);
    }
}
