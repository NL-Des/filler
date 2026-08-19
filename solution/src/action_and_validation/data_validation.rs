pub fn validate_input(player: String, anfield_coordinates: String, anfield_grid: String, piece_numbers: String, piece_grid: String) -> (bool, String) {

    if player.is_empty() {
        return (false, String::from("player is empty."));
    }
    if anfield_coordinates.is_empty(){
        return (false, String::from("anfield coordinates is empty."));
    }
    if !anfield_grid.chars().all(|c| matches!(c, '.' | '$' | 's' | '@' | 'a')) {
        return (false, String::from("anfield grid contains invalid characters."));
    }
    if piece_numbers.is_empty(){
        return (false, String::from("piece numbers is empty."));
    }
    if !piece_grid.chars().all(|c| matches!(c, '.' | '#')) {
        return (false, String::from("piece grid contains invalid characters."));
    }
    (true, String::from("Input is valid."))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_input_test() {
        let (is_valid, message) = validate_input("Player".into(), "AnfieldCoordinates".into(), "AnfieldGrid".into(), "PieceNumbers".into(), "PieceGrid".into());
        assert_eq!(is_valid, true);
        assert_eq!(message, "Input is valid.");

        let (is_valid, message) = validate_input("Player".into(), "".into(), "AnfieldGrid".into(), "PieceNumbers".into(), "PieceGrid".into());
        assert_eq!(is_valid, false);
        assert_eq!(message, "anfield coordinates is empty.");

        let (is_valid, message) = validate_input("Player".into(), "AnfieldCoordinates".into(), "AnfieldGrid".into(), "".into(), "PieceGrid".into());
        assert_eq!(is_valid, false);
        assert_eq!(message, "piece numbers is empty.");

        let (is_valid, message) = validate_input("Player".into(), "AnfieldCoordinates".into(), "AnfieldGrid".into(), "PieceNumbers".into(), "".into());
        assert_eq!(is_valid, false);
        assert_eq!(message, "piece grid is empty.");

        let (is_valid, message) = validate_input("Player".into(), "AnfieldCoordinates".into(), "..$s@x", "PieceNumbers".into(), "PieceGrid".into());
        assert_eq!(is_valid, false);
        assert_eq!(message, "anfield grid contains invalid characters.");

        let (is_valid, message) = validate_input("Player".into(), "AnfieldCoordinates".into(), "AnfieldGrid".into(), "PieceNumbers".into(), ".#x");
        assert_eq!(is_valid, false);
        assert_eq!(message, "piece grid contains invalid characters.");
    }
}