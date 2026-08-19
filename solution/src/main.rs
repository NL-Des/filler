#[path = "input/input.rs"]
mod input;
#[path = "input/input_data_formatting.rs"]
mod input_data_formatting;
#[path = "action_and_validation/data_validation.rs"]
mod data_validation;

use input::read_input;
use input_data_formatting::reading_input;

fn main() {
    let player: char = 'O';
    loop {
        // Lecture de chaque ligne reçue.
        let data_input = read_input();

        let test = reading_input(data_input, player);
    }


}
