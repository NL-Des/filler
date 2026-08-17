use std::io::BufRead;

pub fn read_input_from<R: BufRead>(reader: R) -> Vec<String> {
    reader
        .lines()
        .map(|line| line.expect("filler/solution/src/input/input.rs : Reading input error"))
        .collect()
}

// Il est obligé de passer par cette fonction, lisant l'autre fonction,
// Car sinon on ne peut pas faire le test unitaire.
pub fn read_input() -> Vec<String> {
    read_input_from(std::io::stdin().lock())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn input_test() {
        let test_input = "Hello\nWorld\n!\n";
        let reader = Cursor::new(test_input);

        let input = read_input_from(reader);

        assert_eq!(input.len(), 3);
        assert_eq!(input[0], "Hello");
        assert_eq!(input[1], "World");
        assert_eq!(input[2], "!");
    }
}