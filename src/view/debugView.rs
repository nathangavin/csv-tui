use tui::{
    backend::Backend,
    Frame
};

use crate::model::UtilsModel::{
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
                            f: &mut Frame<B>) {
    
}
