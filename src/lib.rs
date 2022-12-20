pub fn get_input_file() -> String {
    let file = std::env::args().nth(1).expect("No filename specified!");
    std::fs::read_to_string(file).expect("No such file!")
}
