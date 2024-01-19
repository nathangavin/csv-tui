use std::io;
use tui::{
    backend::Backend,
    Terminal};
use crossterm::event::{
        self, 
        KeyCode, 
        Event};

use crate::model::defaultAppModel::DefaultAppModel;

#[derive(Debug, PartialEq, Eq, Hash)]
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
        
        let app_is_saved = app.is_in_saved_state();
        let app_filename = app.get_filename();
        let app_pos = app.get_current_pos();

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
        let cols = usize::from(data_width / 6);
        let page_width = cols;
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

        let rows = usize::from(data_height);
        let page_height = rows;

        let mut max_widths : Vec<usize> = Vec::new();

        //max_widths.push(col_width);
        for col in 1..cols {
            max_widths.push(app.get_max_col_width(col-1));
        } 


        terminal.draw(|f| {

            let app_data = app.get_data();
            render_ui(app_data, 
                      app.get_input(), 
                      app.get_input_mode(), 
                      &running_mode,
                      app_filename, 
                      app_is_saved,
                      app_pos,
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
                        let data_row_pos = (page_height
                                            * app.get_current_page_pos().row())
                                            + app.get_current_pos().row();
                        let data_col_pos = (page_width
                                            * app.get_current_page_pos().col())
                                            + app.get_current_pos().col();
                        app.set_cell_value_current_input(data_row_pos, data_col_pos);
                    },
                    KeyCode::Char('q') => {
                        app.set_input_node(InputMode::Quiting);
                    },
                    KeyCode::Char('s') => {
                        if app_is_saved {
                            app.set_input_node(InputMode::Saved);
                        } else {
                            match app_filename {
                                Some(_) => {
                                    match app.save_data_to_file() {
                                        Ok(_) => {
                                            app.set_saved(true); 
                                            app.set_input_node(InputMode::Saved);
                                        },
                                        Err(_) => {
                                            app.set_saved(false);
                                            app.set_input_node(InputMode::SavedFailed);
                                        }
                                    }
                                },
                                None => {
                                    app.set_input_node(InputMode::Saving);
                                }
                            }
                        }
                        // file saved, message needs to show and then input
                        // change to normal
                    },
                    KeyCode::Char('a') => {
                        app.set_input_node(InputMode::Saving);
                    },
                    KeyCode::Left | KeyCode::Char('h') => {
                        app.decrement_current_pos_col();
                    },
                    KeyCode::Right | KeyCode::Char('l') => {
                        app.increment_current_pos_col();
                    },
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.decrement_current_pos_row();
                    },
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.increment_current_pos_row();
                    },
                    KeyCode::Char('H') => {
                        app.decrement_current_page_pos_col();
                    },
                    KeyCode::Char('L') => {
                        app.increment_current_page_pos_col();
                    },
                    KeyCode::Char('K') => {
                        app.decrement_current_page_pos_row();
                    },
                    KeyCode::Char('J') => {
                        app.increment_current_page_pos_row();
                    },
                    KeyCode::Char('r') => {
                        app.set_input_node(InputMode::SelectingRow);
                    },
                    KeyCode::Char('c') => {
                        app.set_input_node(InputMode::SelectingCol);
                    },
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        app.set_input_to_current_pos();
                        app.set_input_node(InputMode::Normal);
                    },
                    KeyCode::Char(char) => {
                        app.append_char_current_input(char);
                    },
                    KeyCode::Backspace => {
                        app.pop_current_input();
                    },
                    KeyCode::Esc => {
                        app.set_input_node(InputMode::Normal);
                    },
                    _ => {}
                },
                InputMode::Saving => match key.code {
                    KeyCode::Enter => {
                        app.set_filename_to_input();
                        match app.save_data_to_file() {
                            Ok(_) => {
                                app.set_saved(true); 
                                app.set_input_node(InputMode::Saved);
                            },
                            Err(_) => {
                                app.set_saved(false);
                                app.set_input_node(InputMode::SavedFailed);
                            }
                        }
                    },
                    KeyCode::Char(char) => {
                        app.append_char_current_input(char);
                    },
                    KeyCode::Backspace => {
                        app.pop_current_input();
                    },
                    KeyCode::Esc => {
                        app.clear_input();
                        app.set_input_node(InputMode::Normal);
                    },
                    _ => {}
                },
                InputMode::Saved | InputMode::SavedFailed => {
                    app.set_input_node(InputMode::Normal);
                },
                InputMode::Quiting => {
                   if app_is_saved {
                       return Ok(());
                   } else {
                       match key.code {
                           KeyCode::Char('y') | KeyCode::Char('Y') => {
                               app.set_input_node(InputMode::QuitSaving);
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
                            match app.save_data_to_file() {
                                Ok(_) => {
                                    app.set_saved(true);
                                    app.set_input_node(InputMode::Quiting);
                                },
                                Err(_) => {
                                    app.set_saved(false);
                                    app.set_input_node(InputMode::SavedFailed);
                                }
                            }
                        },
                        None => { 
                            match key.code {
                                KeyCode::Enter => {
                                    app.set_filename_to_input();
                                    match app.save_data_to_file() {
                                        Ok(_) => {
                                            app.set_saved(true); 
                                            app.set_input_node(InputMode::Quiting);
                                        },
                                        Err(_) => {
                                            app.set_saved(false);
                                            app.set_input_node(InputMode::SavedFailed);
                                        }
                                    }
                                },
                                KeyCode::Char(char) => {
                                    app.append_char_current_input(char);
                                },
                                KeyCode::Backspace => {
                                    app.pop_current_input();
                                },
                                KeyCode::Esc => {
                                    app.clear_input();
                                    app.set_input_node(InputMode::Normal);
                                },
                                _ => {}
                            }
                        }
                    }
                },
                InputMode::SelectingRow | InputMode::SelectingCol => {
                    match key.code {
                        KeyCode::Char('i') => {
                            app.insert_remove_row_col(
                                InsertMode::Adding
                            );
                            app.set_saved(false);
                            app.set_input_node(InputMode::Normal);
                        },
                        KeyCode::Char('r') => {
                            app.insert_remove_row_col(
                                InsertMode::Removing
                            );
                            app.set_saved(false);
                            app.set_input_node(InputMode::Normal);
                        },
                        KeyCode::Esc => {
                            app.set_input_node(InputMode::Normal);
                        },
                        _ => {} 
                    }
                },
            }
        }
    }
}
