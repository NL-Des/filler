pub fn validate_input(anfield: &str, piece: &str) -> (bool, String) {

    if anfield.is_empty(){
        return (false, String::from("anfield is empty."));
    }
    if !anfield.chars().all(|c| matches!(c, '.' | '$' | 's' | '@' | 'a')) {
        return (false, String::from("anfield contains invalid characters."));
    }

    if piece.is_empty(){
        return (false, String::from("piece is empty."));
    }
    if !piece.chars().all(|c| matches!(c, '.' | '#')) {
        return (false, String::from("piece contains invalid characters."));
    }
    (true, String::from("Input is valid."))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_input_test() {
        let (is_valid, message) = validate_input("..$s@", ".#");
        assert_eq!(is_valid, true);
        assert_eq!(message, "Input is valid.");

        let (is_valid, message) = validate_input("", ".#");
        assert_eq!(is_valid, false);
        assert_eq!(message, "anfield is empty.");

        let (is_valid, message) = validate_input("..$s@", "");
        assert_eq!(is_valid, false);
        assert_eq!(message, "piece is empty.");

        let (is_valid, message) = validate_input("..$s@x", ".#");
        assert_eq!(is_valid, false);
        assert_eq!(message, "anfield contains invalid characters.");

        let (is_valid, message) = validate_input("..$s@", ".#x");
        assert_eq!(is_valid, false);
        assert_eq!(message, "piece contains invalid characters.");
    }
}