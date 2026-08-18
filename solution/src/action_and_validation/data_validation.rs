pub fn validate_input(anfield: &str, piece: &str) -> (bool, String) {

    if anfield.is_empty(){
        return false, String::from("anfield is empty.");
    }
    if !anfield.chars().all(|c| matches!(c, '.' | '$' | 's' | '@' | 'a')) {
        return false, String::from("anfield contains invalid characters.");
    }

    if piece.is_empty(){
        return false, String::from("piece is empty.");
    }
    if !piece.chars().all(|c| matches!(c, '.' | '$' | 's' | '@' | 'a')) {
        return false, String::from("piece contains invalid characters.");
    }
    true
}