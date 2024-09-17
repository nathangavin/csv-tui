use std::io;
use tui::{
    backend::Backend,
    Terminal, Frame};
use crossterm::event::{
        self, 
        KeyCode, 
        Event};

use crate::model::{
    app_state_model::AppStateModel,
    csv_model::CsvModel,
    utils_model::{
        RunningMode,
        InputMode,
        Size, Position
    },
};

pub fn run<B: Backend>(
            app_data: &mut CsvModel,
            app_state: &mut AppStateModel,
            ui_render_function: fn(Vec<Vec<String>>,
                                   &Size,
                                   Size,
                                   Vec<usize>,
                                   &Position,
                                   &Position,
                                   &InputMode,
                                   &RunningMode,
                                   &str,
                                   &Option<String>,
                                   bool,
                                   &mut Frame<B>),
            terminal: &mut Terminal<B>,
            ) -> io::Result<()> {
    
    loop {

        let info_row_height = 1;
        let input_box_height = 3;
        let col_width: usize = 5;
        // Calculating number of columns that can fit on screen
        let border_width = 1;
        let row_num_col_width = col_width as u16 + 1;
        let terminal_width = terminal.size()?.width;
        let width_to_remove = (border_width*2) + row_num_col_width;
        let data_width = if terminal_width > width_to_remove {
            terminal_width - width_to_remove
        } else { 
            0
        };
       
        let terminal_height = terminal.size()?.height;
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

        let grid_size = Size { 
            width: calculate_current_grid_columns(app_state, 
                                                  app_data, 
                                                  data_width as usize), 
            height: data_height as usize 
        };


        let input_mode = app_state.get_input_mode();
        let corner_pos = app_state.get_corner_pos();

        let mut column_widths : Vec<usize> = Vec::new();
        for col in (corner_pos.col)..(corner_pos.col + grid_size.width) {
            column_widths.push(app_data.get_col_max_width(col));
        } 

        let relative_pos = app_state.get_relative_pos();
        let app_is_saved = app_data.is_in_saved_state();
        let app_filename = app_data.get_filename();

        let data_slice = app_data.get_data_segment(&corner_pos, &grid_size);

        terminal.draw(|f| {
            ui_render_function(data_slice, 
                      &grid_size,
                      app_data.get_data_size(),
                      column_widths,
                      &corner_pos,
                      &relative_pos,
                      input_mode,
                      app_state.get_running_mode(),
                      app_state.get_input(),
                      app_filename,
                      app_is_saved,
                      f)
        })?;
        if let Event::Key(key) = event::read()? {
            match input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app_state.set_input_mode(InputMode::Editing);
                        let data_row_pos = corner_pos.row + relative_pos.row;
                        let data_col_pos = corner_pos.col + relative_pos.col;
                        
                        app_state.append_str_current_input(
                            app_data.get_cell_value(data_row_pos, data_col_pos));
                    },
                    KeyCode::Char('q') => {
                        app_state.set_input_mode(InputMode::Quiting);
                    },
                    KeyCode::Char('s') => {
                        if app_is_saved {
                            app_state.set_input_mode(InputMode::Saved);
                        } else {
                            match app_filename {
                                Some(_) => {
                                    match app_data.save_data_to_file() {
                                        Ok(_) => {
                                            app_data.set_saved(true); 
                                            app_state.set_input_mode(InputMode::Saved);
                                        },
                                        Err(_) => {
                                            app_data.set_saved(false);
                                            app_state.set_input_mode(InputMode::SavedFailed);
                                        }
                                    }
                                },
                                None => {
                                    app_state.set_input_mode(InputMode::Saving);
                                }
                            }
                        }
                        // file saved, message needs to show and then input
                        // change to normal
                    },
                    KeyCode::Char('a') => {
                        app_state.set_input_mode(InputMode::Saving);
                    },
                    KeyCode::Left | KeyCode::Char('h') => {
                        app_state.decrement_relative_pos_col();
                    },
                    KeyCode::Right | KeyCode::Char('l') => {
                        app_state.increment_relative_pos_col();
                    },
                    KeyCode::Up | KeyCode::Char('k') => {
                        app_state.decrement_relative_pos_row();
                    },
                    KeyCode::Down | KeyCode::Char('j') => {
                        app_state.increment_relative_pos_row();
                    },
                    KeyCode::Char('H') => {
                        if corner_pos.col > 0 {
                            let prev_grid_size = Size {
                                width: calculate_prev_grid_columns(app_state, 
                                                                   app_data, 
                                                                   data_width as usize),
                                height: data_height as usize
                            };
                            app_state.remove_from_corner_pos_col(prev_grid_size.width);
                        }
                    },
                    KeyCode::Char('L') => {
                        app_state.add_to_corner_pos_col(grid_size.width);
                    },
                    KeyCode::Char('K') => {
                        if corner_pos.row > 0 {
                            let prev_grid_size = Size {
                                width: calculate_prev_grid_columns(app_state, 
                                                                   app_data, 
                                                                   data_width as usize),
                                height: data_height as usize
                            };
                            app_state.remove_from_corner_pos_row(prev_grid_size.height);
                        }
                    },
                    KeyCode::Char('J') => {
                        app_state.add_to_corner_pos_row(grid_size.height);
                    },
                    KeyCode::Char('r') => {
                        app_state.set_input_mode(InputMode::SelectingRow);
                    },
                    KeyCode::Char('c') => {
                        app_state.set_input_mode(InputMode::SelectingCol);
                    },
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        /*
                         * draing the current input value from the app state, 
                         * then set the value of the current cell to the value
                         * of the input.
                         */
                        let row = app_state.get_corner_pos().row 
                            + app_state.get_relative_pos().row;
                        let col = app_state.get_corner_pos().col
                            + app_state.get_relative_pos().col;
                        app_data.set_cell_value(row, col, app_state.drain_input());
                        app_state.set_input_mode(InputMode::Normal);
                    },
                    KeyCode::Char(char) => {
                        app_state.append_char_current_input(char);
                    },
                    KeyCode::Backspace => {
                        app_state.pop_current_input();
                    },
                    KeyCode::Esc => {
                        app_state.set_input_mode(InputMode::Normal);
                    },
                    _ => {}
                },
                InputMode::Saving => match key.code {
                    KeyCode::Enter => {
                        app_data.set_filename(app_state.drain_input());
                        match app_data.save_data_to_file() {
                            Ok(_) => {
                                app_data.set_saved(true);
                                app_state.set_input_mode(InputMode::Saved);
                            },
                            Err(_) => {
                                app_data.set_saved(false);
                                app_state.set_input_mode(InputMode::SavedFailed);
                            }
                        }
                    },
                    KeyCode::Char(char) => {
                        app_state.append_char_current_input(char);
                    },
                    KeyCode::Backspace => {
                        app_state.pop_current_input();
                    },
                    KeyCode::Esc => {
                        app_state.clear_input();
                        app_state.set_input_mode(InputMode::Normal);
                    },
                    _ => {}
                },
                InputMode::Saved | InputMode::SavedFailed => {
                    app_state.set_input_mode(InputMode::Normal);
                },
                InputMode::Quiting => {
                   if app_is_saved {
                       return Ok(());
                   } else {
                       match key.code {
                           KeyCode::Char('y') | KeyCode::Char('Y') => {
                               app_state.set_input_mode(InputMode::QuitSaving);
                           },
                           KeyCode::Char('n') | KeyCode::Char('N') => {
                               return Ok(());
                           },
                           _ => {}
                       } 
                   } 
                },
                InputMode::QuitSaving => {
                    match app_filename {
                        Some(_) => {
                            match app_data.save_data_to_file() {
                                Ok(_) => {
                                    app_data.set_saved(true);
                                    app_state.set_input_mode(InputMode::Quiting);
                                },
                                Err(_) => {
                                    app_data.set_saved(false);
                                    app_state.set_input_mode(InputMode::SavedFailed);
                                }
                            }
                        },
                        None => { 
                            match key.code {
                                KeyCode::Enter => {
                                    app_data.set_filename(app_state.drain_input());
                                    match app_data.save_data_to_file() {
                                        Ok(_) => {
                                            app_data.set_saved(true);
                                            app_state.set_input_mode(InputMode::Quiting);
                                        },
                                        Err(_) => {
                                            app_data.set_saved(false);
                                            app_state.set_input_mode(InputMode::SavedFailed);
                                        }
                                    }
                                },
                                KeyCode::Char(char) => {
                                    app_state.append_char_current_input(char);
                                },
                                KeyCode::Backspace => {
                                    app_state.pop_current_input();
                                },
                                KeyCode::Esc => {
                                    app_state.clear_input();
                                    app_state.set_input_mode(InputMode::Normal);
                                },
                                _ => {}
                            }
                        }
                    }
                },
                InputMode::SelectingRow => {
                    let row = app_state.get_corner_pos().row
                        + app_state.get_relative_pos().row;
                    match key.code {
                        KeyCode::Char('i') => {
                            app_data.insert_row(row);
                            app_data.set_saved(false);
                            app_state.set_input_mode(InputMode::Normal);
                        },
                        KeyCode::Char('r') => {
                            app_data.remove_row(row);
                            app_data.set_saved(false);
                            app_state.set_input_mode(InputMode::Normal);
                        },
                        KeyCode::Esc => {
                            app_state.set_input_mode(InputMode::Normal);
                        },
                        _ => {}
                    }
                },
                InputMode::SelectingCol => {
                    let col = app_state.get_corner_pos().col
                        + app_state.get_relative_pos().col;
                    match key.code {
                        KeyCode::Char('i') => {
                            app_data.insert_col(col);
                            app_data.set_saved(false);
                            app_state.set_input_mode(InputMode::Normal);
                        },
                        KeyCode::Char('r') => {
                            app_data.remove_col(col);
                            app_data.set_saved(false);
                            app_state.set_input_mode(InputMode::Normal);
                        },
                        KeyCode::Esc => {
                            app_state.set_input_mode(InputMode::Normal);
                        },
                        _ => {} 
                    }
                },
            }
        }
    }

    fn calculate_current_grid_columns(app_state: &AppStateModel,
                                      app_data: &CsvModel,
                                      area_width: usize) -> usize {

        let mut num_cols = 0;
        let mut total_widths = 0;
        let mut current_col = app_state.get_corner_pos().col;

        loop {
            total_widths += app_data.get_col_max_width(current_col) + 1;
            if total_widths < area_width {
                num_cols += 1;
                current_col += 1;
            } else {
                break;
            }
        }

        return num_cols;
    }

    fn calculate_prev_grid_columns(app_state: &AppStateModel, 
                                        app_data: &CsvModel, 
                                        area_width: usize) -> usize {
        let mut num_cols = 0;
        let mut total_widths = 0;
        let mut current_col = app_state.get_corner_pos().col;

        loop {
            total_widths += app_data.get_col_max_width(current_col) + 1;
            if total_widths < area_width && current_col > 0 {
                num_cols += 1;
                current_col -= 1;
            } else {
                break;
            }
        }

        return num_cols;
    }
}
