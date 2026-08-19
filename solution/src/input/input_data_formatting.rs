pub fn reading_input(data_input: Vec<String>, mut player: char) -> (char, char, char, Vec<String>, char, char, Vec<String>) {
    let mut x:char = '0';
    let mut y:char = '0';
    let mut table:Vec<String> = Vec::new();
    let mut x2:char = '0';
    let mut y2:char = '0';
    let mut piece:Vec<String> = Vec::new();
    
    // Lecture de chaque ligne.
    for line in data_input {
        // Si la ligne ne contient que des espaces, on l'ignore.

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

        // Si la ligne donne les coordonnées du tableau de jeu.
        // Elle ne sera pas recherchée grâce aux différents filtres dans les if.

        // Si la ligne contient des caractères faisant partie du tableau de jeu.
        if line.contains(&['.', 's', '$', 'a', '@', ' ']) && line.contains(char::is_numeric) && line.len() == x as usize {
            table.push(line.clone());
        }

        // Si la ligne donne les coordonnées de la pièce.
        if line.starts_with("Piece") {
            let parts2: Vec<&str> = line.split_whitespace().collect();
            if parts2.len() >= 3 {
                x2 = parts2[1].chars().next().unwrap_or('0');
                y2 = parts2[2].chars().next().unwrap_or('0');
            }
        }

        // Si la ligne contient des caractères spécifiques à la pièce à poser.
        if line.contains('0') && line.contains('.') {
            piece.push(line.clone());
        }

    }

        (player, x, y, table, x2, y2, piece)
    }
