use std::{io::{self, Error as IO_Error, ErrorKind}, fs };
use tui::{
    backend::Backend,
    Terminal};
use crossterm::event::{
        self, 
        KeyCode, 
        Event};

use crate::model::defaultAppModel::DefaultAppModel;

pub enum InputMode {
    Normal,
    Editing,
    SelectingRow,
    SelectingCol,
    Saving,
    Quiting,
    QuitSaving,
    Saved,
    SavedFailed
}
pub enum InsertMode {
    Adding,
    Removing
}
pub enum RunningMode {
    Normal,
    Debug
}

use crate::view::defaultView::render_ui;

pub fn run<B: Backend>(
            app: &mut DefaultAppModel,
            terminal: &mut Terminal<B>,
            running_mode: RunningMode
            ) -> io::Result<()> {

    loop {
        terminal.draw(|f| {

            let info_row_height = 1;
            let input_box_height = 3;
            let col_width: usize = 5;
            // Calculating number of columns that can fit on screen
            let border_width = 1;
            let row_num_col_width = col_width as u16 + 1;
            let terminal_width = f.size().width;
            let width_to_remove = (border_width*2) + row_num_col_width;
            let data_width = if terminal_width > width_to_remove {
                terminal_width - width_to_remove
            } else { 
                0
            };
            let cols = usize::from(data_width / 6);
            app.set_page_size_width(cols);
            let terminal_height = f.size().height;
            let index_row_height = 1;
            let height_to_remove = info_row_height 
                                    + input_box_height 
                                    + (border_width * 2)
                                    + index_row_height;
            let data_height = if terminal_height > height_to_remove {
                terminal_height - height_to_remove
            } else {
                0
            };

            let rows = usize::from(data_height);
            app.set_page_size_height(rows);

            let max_widths : Vec<usize> = Vec::new();

            max_widths.push(col_width);
            for col in 1..cols {
                max_widths.push(app.get_max_col_width(col-1));
            } 

            render_ui(app.get_data(), 
                      app.get_data_width(),
                      app.get_input(), 
                      app.get_input_mode(), 
                      running_mode,
                      app.get_filename(), 
                      app.is_in_saved_state(),
                      app.get_current_pos(),
                      app.get_current_page_pos(),
                      max_widths,
                      f)
            //render_ui(f, &running_mode)
        })?;
        if let Event::Key(key) = event::read()? {
            match app.get_input_mode() {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.set_input_node(InputMode::Editing);
                        let data_row_pos = (self.page_size.height 
                                            * self.page_pos.row) 
                                            + self.pos.row;
                        let data_col_pos = (self.page_size.width
                                            * self.page_pos.col) 
                                            + self.pos.col;
                        match self.data.get(data_row_pos) {
                            Some(row) => {
                                match row.get(data_col_pos) {
                                    Some(cell) => {
                                        self.input.push_str(cell)
                                    },
                                    None => {}
                                }
                            },
                            None => {} 
                        }
                    },
                    KeyCode::Char('q') => {
                        self.input_mode = InputMode::Quiting;
                    },
                    KeyCode::Char('s') => {
                        if self.saved == true {
                            self.input_mode = InputMode::Saved;
                        } else {
                            match &self.filename {
                                Some(_) => {
                                    match self.save_data_to_file() {
                                        Ok(_) => {
                                            self.saved = true;
                                            self.input_mode = 
                                                InputMode::Saved;
                                        },
                                        Err(_) => {
                                            self.input_mode = 
                                                InputMode::SavedFailed;
                                        }
                                    }
                                },
                                None => { 
                                    self.input_mode = InputMode::Saving;
                                }
                            }
                        }
                        // file saved, message needs to show and then input
                        // change to normal
                    },
                    KeyCode::Char('a') => {
                        self.input_mode = InputMode::Saving;
                    },
                    KeyCode::Left | KeyCode::Char('h') => {
                        if self.pos.col > 0 {
                            self.pos.col -= 1;
                        }
                    },
                    KeyCode::Right | KeyCode::Char('l') => {
                        self.pos.col += 1;
                    },
                    KeyCode::Up | KeyCode::Char('k') => {
                        if self.pos.row > 0 {
                            self.pos.row -= 1;
                        }
                    },
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.pos.row += 1;
                    },
                    KeyCode::Char('L') => {
                        self.page_pos.col += 1;
                    },
                    KeyCode::Char('H') => {
                        if self.page_pos.col > 0 {
                            self.page_pos.col -= 1;
                        }
                    },
                    KeyCode::Char('J') => {
                        self.page_pos.row += 1;
                    },
                    KeyCode::Char('K') => {
                        if self.page_pos.row > 0 {
                            self.page_pos.row -= 1;
                        }
                    },
                    KeyCode::Char('r') => {
                        self.input_mode = InputMode::SelectingRow;
                    },
                    KeyCode::Char('c') => {
                        self.input_mode = InputMode::SelectingCol;
                    },
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        let current_input = self.input.drain(..).collect();
                        self.add_value_to_cell(current_input);
                        self.saved = false;
                        self.input_mode = InputMode::Normal;
                    },
                    KeyCode::Char(char) => {
                        self.input.push(char);
                    },
                    KeyCode::Backspace => {
                        self.input.pop();
                    },
                    KeyCode::Esc => {
                        self.input_mode = InputMode::Normal;
                    },
                    _ => {}
                },
                InputMode::Saving => match key.code {
                    KeyCode::Enter => {
                        let current_input = self.input.drain(..).collect();
                        self.filename = Some(current_input);
                        match self.save_data_to_file() {
                            Ok(_) => {
                                self.saved = true;
                                self.input_mode = InputMode::Saved;
                            },
                            Err(_) => {
                                self.saved = false; 
                                self.input_mode = InputMode::SavedFailed;
                            }
                        }
                    },
                    KeyCode::Char(char) => {
                        self.input.push(char);
                    },
                    KeyCode::Backspace => {
                        self.input.pop();
                    },
                    KeyCode::Esc => {
                        self.input.clear();
                        self.input_mode = InputMode::Normal;
                    },
                    _ => {}
                },
                InputMode::Saved | InputMode::SavedFailed => {
                    self.input_mode = InputMode::Normal;
                },
                InputMode::Quiting => {
                   if self.saved == true {
                       return Ok(());
                   } else {
                       match key.code {
                           KeyCode::Char('y') => {
                               self.input_mode = InputMode::QuitSaving;
                           },
                           KeyCode::Char('n') => {
                               return Ok(());
                           },
                           _ => {}
                       } 
                   } 
                },
                InputMode::QuitSaving => {
                    match &self.filename {
                        Some(_) => {
                            match self.save_data_to_file() {
                                Ok(_) => {
                                    self.saved = true;
                                    self.input_mode = 
                                        InputMode::Quiting;
                                },
                                Err(_) => {
                                    self.input_mode = 
                                        InputMode::SavedFailed;
                                }
                            }
                        },
                        None => { 
                            match key.code {
                                KeyCode::Enter => {
                                    let current_input = self.input
                                                        .drain(..)
                                                        .collect();
                                    self.filename = Some(current_input);
                                    match self.save_data_to_file() {
                                        Ok(_) => {
                                            self.saved = true;
                                            self.input_mode = 
                                                InputMode::Quiting;
                                        },
                                        Err(_) => {
                                            self.saved = false; 
                                            self.input_mode = 
                                                InputMode::SavedFailed;
                                        }
                                    }
                                },
                                KeyCode::Char(char) => {
                                    self.input.push(char);
                                },
                                KeyCode::Backspace => {
                                    self.input.pop();
                                },
                                KeyCode::Esc => {
                                    self.input.clear();
                                    self.input_mode = InputMode::Normal;
                                },
                                _ => {}
                            }
                        }
                    }
                },
                InputMode::SelectingRow | InputMode::SelectingCol => {
                    match key.code {
                        KeyCode::Char('i') => {
                            self.insert_remove_row_col(
                                InsertMode::Adding
                            );
                            self.saved = false;
                            self.input_mode = InputMode::Normal;
                        },
                        KeyCode::Char('r') => {
                            self.insert_remove_row_col(
                                InsertMode::Removing
                            );
                            self.saved = false;
                            self.input_mode = InputMode::Normal;
                        },
                        KeyCode::Esc => {
                            self.input_mode = InputMode::Normal;
                        },
                        _ => {} 
                    }
                },
            }
        }
    }
}
