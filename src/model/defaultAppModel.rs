use std::{io::{self, Error as IO_Error, ErrorKind}, fs };
use crate::controller::defaultController::InputMode;
use crate::controller::defaultController::InsertMode;

pub struct Position {
    pub row: usize,
    pub col: usize
}

pub struct Size {
    width: usize,
    height: usize
}

pub struct DefaultAppModel {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    data: Vec<Vec<String>>,
    pos: Position,
    saved: bool,
    filename: Option<String>,
    page_pos: Position,
    page_size: Size
}

impl Default for DefaultAppModel {
    fn default() -> DefaultAppModel {
        DefaultAppModel {
            input: String::new(),
            input_mode: InputMode::Normal,
            data: Vec::new(),
            pos: Position { row: 0, col: 0 },
            saved: true,
            filename: None,
            page_pos: Position { row: 0, col: 0 },
            page_size: Size { width: 0, height: 0 } 
        }
    }
}

impl DefaultAppModel {
    pub fn load_file_into_app(filename: String) -> Result<DefaultAppModel, io::Error> {
        let mut app = DefaultAppModel::default();
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(&filename)?;
        app.filename = Some(filename); 
       
        for row in reader.records() {
            app.data.push(row.unwrap().iter().map(|cell_value| {
                String::from(cell_value)
            }).collect());
        }

        Ok(app)
    }

    pub fn set_page_size_width(&mut self, width: usize) {
        self.page_size.width = width;
    }

    pub fn set_page_size_height(&mut self, height: usize) {
        self.page_size.height = height;
    }

    pub fn get_filename(&self) -> Option<String> {
        self.filename
    }

    pub fn get_current_pos(&self) -> Position {
        self.pos
    }

    pub fn get_current_page_pos(&self) -> Position {
        self.page_pos
    }

    fn insert_remove_row_col(&mut self, insert_mode: InsertMode) {
        match self.input_mode {
            InputMode::SelectingCol => {
                let col_pos = (self.page_size.width * self.page_pos.col)
                                    + self.pos.col;
                match insert_mode {
                    InsertMode::Adding => {
                        self.insert_col(col_pos);
                    },
                    InsertMode::Removing => {
                        self.remove_col(col_pos);
                    }
                }
            },
            InputMode::SelectingRow => {
                let row_pos = (self.page_size.height * self.page_pos.row)
                                    + self.pos.row;
                match insert_mode {
                    InsertMode::Adding => {
                        self.insert_row(row_pos);
                    },
                    InsertMode::Removing => {
                        self.remove_row(row_pos);
                    }
                }
            },
            _ => {}
        }
    }

    pub fn get_data(&self) -> Vec<Vec<String>> {
        self.data
    }

    pub fn get_input(&self) -> &str {
        self.input.as_str()
    }

    pub fn has_filename(&self) -> bool {
        match self.filename {
            Some(_) => true,
            None => false
        }
    }
    
    pub fn is_in_saved_state(&self) -> bool {
        self.saved 
    }

    fn insert_row(&mut self, row_pos: usize) {
        if row_pos < self.data.len() {
            self.data.insert(row_pos, (0..self.get_max_row_length())
                                        .into_iter()
                                        .map(|_| String::from(""))
                                        .collect());
        } 
    }

    fn get_max_row_length(&self) -> usize {
        let mut max_length = 0;
        for row in self.data.iter() {
            if max_length < row.len() {
                max_length = row.len();
            }
        };
        max_length
    }

    fn remove_row(&mut self, row_pos: usize) {
        if row_pos < self.data.len() {
            self.data.remove(row_pos);
        }
    }

    fn insert_col(&mut self, col_pos: usize) {
        for row in self.data.iter_mut() {
            if col_pos < row.len() {
                row.insert(col_pos, String::from(""));
            }
        }
    }
   
    fn remove_col(&mut self, col_pos: usize) {
        for row in self.data.iter_mut() {
            if col_pos < row.len() {
                row.remove(col_pos);
            }
        }
    }

    fn add_value_to_cell(&mut self, input: String) {
        let row_pos = (self.page_size.height * self.page_pos.row) 
                        + self.pos.row;
        let row: &mut Vec<String> = match self.data.get_mut(row_pos) {
            Some(data_row) => data_row,
            None => {
                for _ in 0..=row_pos {
                    self.data.push(Vec::new());
                }
                self.data.get_mut(row_pos).unwrap()
            }
        };
        let col_pos = (self.page_size.width * self.page_pos.col)
                        + self.pos.col;
        let cell = match row.get_mut(col_pos) {
            Some(cell_data) => cell_data,
            None => {
                for _ in 0..=col_pos {
                    row.push(String::new());
                }
                row.get_mut(col_pos).unwrap()
            }
        };
        *cell = input;
        self.remove_unneeded_rows();
    }

    pub fn get_max_col_width(&self, col: usize) -> usize {
        let mut max_width = 5;

        for row in self.data.iter() {
            match row.get(col) {
                Some(cell_value) => {
                    if cell_value.len() > max_width {
                        max_width = cell_value.len();
                    }
                },
                None => {}
            }
        }
       max_width 
    }
    
    pub fn get_data_width(&self) -> usize {
        match self.data.get(0) {
            Some(row) => row.len(),
            None => 0 
        }
    }

    fn remove_unneeded_rows(&mut self) {
        let mut largest_row_col = (0,0);
        for (row_pos,row) in self.data.iter().enumerate() {
            let mut has_data = false;
            for (col_pos,col) in row.iter().enumerate() {
                if col.len() > 0 {
                    largest_row_col.1 = if col_pos > largest_row_col.1 {
                        col_pos
                    } else {
                        largest_row_col.1
                    };
                    has_data = true;
                }
            }
            if has_data {
                largest_row_col.0 = if row_pos > largest_row_col.0 {
                    row_pos
                } else {
                    largest_row_col.0
                };
            }
        }
        for pos in ((largest_row_col.0 + 1)..self.data.len()).rev() {
            if pos < self.data.len() {
                self.data.remove(pos);
            }
        }
        for row in self.data.iter_mut() {
            for pos in ((largest_row_col.1 + 1)..row.len()).rev() {
                if pos < row.len() {
                    row.remove(pos);
                }
            }
        }
    }

    fn save_data_to_file(&self) -> std::io::Result<()>  {
        match &self.filename {
            Some(name) => {
                if name.ends_with(".csv") {
                    fs::write(name, self.create_csv_string())?;
                } else {
                    let new_name = format!("{}{}", name, ".csv");
                    fs::write(new_name, self.create_csv_string())?;
                }
            },
            None => {
                return Err(IO_Error::new(ErrorKind::Other, "filename not set"));
            }
        }
        Ok(())
    }

    fn create_csv_string(&self) -> String {
        let lengths: Vec<usize> = self.data.iter().map(|row| row.len()).collect();
        let num_cols = match lengths.iter().max() {
            Some(value) => *value,
            None => 0
        };
        let output = self.data.iter().fold(String::new(), |mut sum, row| {
            let mut row_value = row.iter().fold(
                String::new(), 
                |mut row_sum, cell| {
                    row_sum.push_str(cell);
                    row_sum.push(',');
                    row_sum
                });
            if row.len() < num_cols {
                for _ in row.len()..num_cols {
                    row_value.push(',');
                }
            }
            row_value.pop();
            row_value.push('\n');
            sum.push_str(&row_value);
            sum
        });
        
        output
    }

    pub fn get_input_mode(&self) -> InputMode {
        self.input_mode
    }

    pub fn set_input_node(&mut self, input_node: InputMode) {
        self.input_mode = input_node;
    }

}
