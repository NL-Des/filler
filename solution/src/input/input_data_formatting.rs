pub fn format_input(data_input: Vec<String>) -> (String, String) {
    let anfield = data_input[0].clone();
    let piece = data_input[1].clone();
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