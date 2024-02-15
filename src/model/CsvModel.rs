use std::{io::{self, Error as IO_Error, ErrorKind}, fs, vec };

use super::UtilsModel::{
    Size,
    Position
};

pub struct CsvModel {
    data: Vec<Vec<String>>,
    saved: bool,
    filename: Option<String>,
}

impl Default for CsvModel {
    fn default() -> Self {
        CsvModel {
            data: Vec::new(),
            saved: true,
            filename: None,
        }
    }
}

impl CsvModel {
    pub fn load_file(filename: String) -> Result<CsvModel, io::Error> {
        let mut csv_model = CsvModel::default();
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(&filename)?;
        csv_model.filename = Some(filename); 
       
        for row in reader.records() {
            csv_model.data.push(row.unwrap().iter().map(|cell_value| {
                String::from(cell_value)
            }).collect());
        }

        Ok(csv_model)
    }

    pub fn get_filename(&self) -> &Option<String> {
        &self.filename
    }

    pub fn set_filename(&mut self, filename: String) {
        self.filename = Some(filename);
    }

    pub fn _get_data(&self) -> &Vec<Vec<String>> {
        &self.data
    }

    pub fn is_in_saved_state(&self) -> bool {
        self.saved 
    }

    pub fn set_saved(&mut self, is_saved: bool) {
        self.saved = is_saved;
    }
 
    pub fn get_data_size(&self) -> Size {
        let width = self.data.len();
        let height = match self.data.get(0) {
            Some(row) => row.len(),
            None => 0
        };

        Size {
            width,
            height
        }
    }

    pub fn insert_row(&mut self, row_pos: usize) {
        if row_pos < self.data.len() {
            self.data.insert(row_pos, (0..self.get_max_row_length())
                                        .into_iter()
                                        .map(|_| String::from(""))
                                        .collect());
        } 
    }


    pub fn remove_row(&mut self, row_pos: usize) {
        if row_pos < self.data.len() {
            self.data.remove(row_pos);
        }
    }

    pub fn insert_col(&mut self, col_pos: usize) {
        for row in self.data.iter_mut() {
            if col_pos < row.len() {
                row.insert(col_pos, String::from(""));
            }
        }
    }
   
    pub fn remove_col(&mut self, col_pos: usize) {
        for row in self.data.iter_mut() {
            if col_pos < row.len() {
                row.remove(col_pos);
            }
        }
    }

    pub fn get_data_segment(&self, 
                            corner_pos: &Position, 
                            grid_size: &Size) -> Vec<Vec<String>> {

        let current_data_height = self.data.len();
        let current_data_width = match self.data.get(0) {
            Some(row) => row.len(),
            None => 0
        };

        let high_row = match (corner_pos.row + grid_size.height) < current_data_height {
            true => corner_pos.row + grid_size.height,
            false => current_data_height
        };
        let high_col = match (corner_pos.col + grid_size.width) < current_data_width {
            true => corner_pos.col + grid_size.width,
            false => current_data_width
        };

        let mut data_segment : Vec<Vec<String>> = Vec::new();
        for row_i in corner_pos.row..high_row {
            let mut new_row = Vec::new();
            match self.data.get(row_i) {
                Some(row) => {
                    for col_i in corner_pos.col..high_col {
                        match row.get(col_i) {
                            Some(cell) => {
                                new_row.push(cell.clone());
                            },
                            None => {}
                        }
                    } 
                },
                None => {}
            }
            data_segment.push(new_row);
        }

        data_segment
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

    /// This function sets the cell value to the value of input, at the position
    /// defined by the row and column parameters. 
    ///
    /// Mutates the CsvModel by changing the specified cell, as well as setting 
    /// the saved flag to false.
    pub fn set_cell_value(&mut self, row: usize, col: usize, input: String) {
        /*
         * get the row to be edited. if the row does not exist, all rows up 
         * to and including the row number needs to be populated with empty 
         * rows.
         */
        for pos in 0..=row {
            match self.data.get_mut(pos) {
                Some(data_prev_row) => {
                    if data_prev_row.len() < col + 1 {
                        let diff = col - data_prev_row.len() + 1;
                        for _ in 0..diff {
                            data_prev_row.push(String::new());
                        }
                    }
                },
                None => {
                    self.data.push(vec![String::new();col + 1]);
                }
            };
        }
        let row: &mut Vec<String> = match self.data.get_mut(row) {
            Some(data_row) => data_row,
            None => {
                for _ in 0..=row {
                    self.data.push(vec![String::new();col + 1]);
                }
                self.data.get_mut(row).unwrap()
            }
        };
        /*
         * same as above, need to retrieve the cell to be edited, and if the 
         * cell does not exist, populate the row so that all cells up to the 
         * required one now exist. 
         */
        let cell = match row.get_mut(col) {
            Some(cell_data) => cell_data,
            None => {
                let diff = col - row.len() + 1;
                for _ in 0..diff {
                    row.push(String::new());
                }
                row.get_mut(col).unwrap()
            }
        };
        *cell = input;
        self.remove_unneeded_rows();
        self.saved = false;
    }

    pub fn get_cell_value(&self, row: usize, col: usize) -> &str {
        match self.data.get(row) {
            Some(row_val) => {
                match row_val.get(col) {
                    Some(cell_val) => &cell_val[..],
                    None => ""
                }
            },
            None => ""
        }
    }

    pub fn get_col_max_width(&self, col: usize) -> usize {
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
    
    /// This function finds the furthest position in the data that contains a 
    /// value, and then removes all rows and columns greater than this position.
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

    pub fn save_data_to_file(&self) -> std::io::Result<()>  {
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
}
