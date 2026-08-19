use std::io::{BufRead};

pub fn read_input() -> Vec<String> {
    let stdin= std::io::stdin().lock();
    stdin.lines()
        .map(|line| line.expect("End of game, or Failed to read line."))
        .collect()
}