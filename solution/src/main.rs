#[path = "input/input.rs"]
mod input;
#[path = "input/input_data_formatting.rs"]
mod input_data_formatting;
#[path = "action_and_validation/data_validation.rs"]
mod data_validation;

use input::read_input;
use input_data_formatting::format_input;
use data_validation::validate_input;

fn main() {
    let data_input = read_input();
    let (anfield, piece) = format_input(data_input);
    let (is_input_ok, error_message) = validate_input(&anfield, &piece);

    if is_input_ok == false {
        println!("{}",error_message);
        return;
    }


}
