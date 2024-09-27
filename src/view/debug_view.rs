use tui::{
    backend::Backend,
    Frame
};

use crate::model::utils_model::{
    Position,
    Size,
    InputMode,
    RunningMode
};

pub fn render_ui<B: Backend>(data_slice: Vec<Vec<String>>,
                            grid_size: &Size,
                            data_size: Size,
                            column_widths: Vec<usize>,
                            corner_pos: &Position,
                            relative_pos: &Position,
                            input_mode: &InputMode,
                            running_mode: &RunningMode,
                            current_input: &str,
                            filename: &Option<String>,
                            is_saved: bool,
                            _f: &mut Frame<B>) {
    println!("{:?}", filename);
    println!("{:?}", is_saved);
    println!("{:?}", input_mode);
    println!("{:?}", running_mode);
    println!("{:?}", corner_pos);
    println!("{:?}", relative_pos);
    println!("{:?}", column_widths);
    println!("{:?}", current_input);
    println!("{:?}", data_size);
    println!("{:?}", grid_size);
    println!("{:?}", data_slice);
    todo!();
}
