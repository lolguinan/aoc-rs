pub fn load_input(day: isize) -> String {
    std::fs::read_to_string(format!("inputs/{day:0>2}.txt")).expect("Could not read input file.")
}
