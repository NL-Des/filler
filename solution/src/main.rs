#[path = "input/input.rs"]
mod input;
#[path = "input/input_data_formatting.rs"]
mod input_data_formatting;

fn main() {
    data_input := read_input_and_process();
    anfield, piece := format_input(data_input);
    is_input_ok, error_message := validate_input(anfield, piece);
    
    if is_input_ok == false {
        println!(error_message);
        return;
    }

    
}
