#[path = "input/input.rs"]
mod input;
#[path = "input/input_data_formatting.rs"]
mod input_data_formatting;


use input::read_input;
use input_data_formatting::reading_input;

fn main() {
    let player: char = 'O';
    loop {
        // Lecture de chaque ligne reçue.
        let data_input = read_input();

        // Découpage des données reçues.
        let (player, x, y, table, x2, y2, piece) = reading_input(data_input, player);
    
        
    }


}
