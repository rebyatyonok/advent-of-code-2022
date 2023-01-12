#[derive(Debug)]
struct Packet {
    data: Vec<char>,
    pointer: usize,
}

impl From<&str> for Packet {
    fn from(str: &str) -> Self {
        let data = str.chars().collect();

        Packet { data, pointer: 0 }
    }
}

fn main() {
    let file = advent_of_code_2022::get_input_file();
    file.split("\n\n")
        .map(|pair| pair.lines().map(Packet::from).collect::<Vec<Packet>>())
        .for_each(|pair| println!("{:?}", pair));
}
