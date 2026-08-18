pub fn format_input(input: Vec<String>) -> (String, String) {
    let anfield = input[0].clone();
    let piece = input[1].clone();
    (anfield, piece)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn format_input_test() {
        let input = vec![
            String::from("Anfield"),
            String::from("Piece"),
        ];
        let (anfield, piece) = format_input(input);
        assert_eq!(anfield, "Anfield");
        assert_eq!(piece, "Piece");
    }
}