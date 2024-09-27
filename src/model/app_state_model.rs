use crate::model::utils_model::{
    Position,
    InputMode,
    RunningMode
};

pub struct AppStateModel {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    running_mode: RunningMode,
    corner_pos: Position,
    relative_pos: Position
}

impl Default for AppStateModel {
    fn default() -> AppStateModel {
        AppStateModel {
            input: String::new(),
            input_mode: InputMode::Normal,
            running_mode: RunningMode::Normal,
            corner_pos: Position { row: 0, col: 0 },
            relative_pos: Position { row: 0, col: 0 },
        }
    }
}

impl AppStateModel {

    pub fn from_running_mode(running_mode: &RunningMode) -> AppStateModel {
        let mut state = AppStateModel::default();

        match running_mode {
            RunningMode::Normal => {},
            RunningMode::Debug => {
                state.running_mode = RunningMode::Debug;
            }
        };
        state
    }

    pub fn get_relative_pos(&self) -> Position {
        self.relative_pos
    }

    pub fn increment_relative_pos_row(&mut self) {
        self.relative_pos.row += 1;
    }
    
    pub fn decrement_relative_pos_row(&mut self) {
        if self.relative_pos.row > 0 {
            self.relative_pos.row -= 1;
        }
    }

    pub fn increment_relative_pos_col(&mut self) {
        self.relative_pos.col += 1;
    }
    
    pub fn decrement_relative_pos_col(&mut self) {
        if self.relative_pos.col > 0 {
            self.relative_pos.col -= 1;
        }
        
    }

    pub fn get_corner_pos(&self) -> Position {
        self.corner_pos
    }

    pub fn add_to_corner_pos_row(&mut self, n: usize) {
        self.corner_pos.row += n;
    }

    pub fn remove_from_corner_pos_row(&mut self, n: usize) {
        if self.corner_pos.row > n {
            self.corner_pos.row -= n;
        } else {
            self.corner_pos.row = 0;
        }
    }

    pub fn add_to_corner_pos_col(&mut self, n: usize) {
        self.corner_pos.col += n;
    }

    pub fn remove_from_corner_pos_col(&mut self, n: usize) {
        if self.corner_pos.col > n {
            self.corner_pos.col -= n;
        } else {
            self.corner_pos.col = 0;
        }
    }

    pub fn append_str_current_input(&mut self, string_value : &str) {
        self.input.push_str(&string_value);
    }

    pub fn append_char_current_input(&mut self, char_value : char) {
        self.input.push(char_value);
    }

    pub fn pop_current_input(&mut self) -> Option<char> {
        self.input.pop()
    }

    pub fn drain_input(&mut self) -> String {
        self.input.drain(..).collect()
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
    }

    pub fn get_input(&self) -> &str {
        self.input.as_str()
    }

    pub fn get_input_mode(&self) -> &InputMode {
        &self.input_mode
    }

    pub fn set_input_mode(&mut self, input_node: InputMode) {
        self.input_mode = input_node;
    }

    pub fn get_running_mode(&self) -> &RunningMode {
        &self.running_mode
    }
}
