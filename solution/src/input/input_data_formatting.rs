pub fn reading_input(data_input: Vec<String>, mut player: char) -> (char, char, char) {
    let mut x:char = '0';
    let mut y:char = '0';
    
    // Lecture de chaque ligne.
    for line in data_input {

        // Si la ligne donne l'information si nous sommes le joueur 1 ou 2.
        if line.starts_with("$$$ exec p") {
            // .find trouve l'information.
            // Some est le contenaire qui la récupère, et la met dans la variable c.
            if let Some(c) = line.chars().find(|c| c.is_ascii_digit()) {
                player = c;
            }
        }

        // Si la ligne donne les coordonnées de l'Anfield.
        if line.starts_with("Anfield") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                x = parts[1].chars().next().unwrap_or('0');
                y = parts[2].chars().next().unwrap_or('0');
            }
        }
    }
    (player, x, y)
}