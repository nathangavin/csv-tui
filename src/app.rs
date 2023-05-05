use std::{io::{self, Error as IO_Error, ErrorKind}, vec, fs };
use tui::{
    backend::
        Backend, 
    Terminal, 
    widgets::{
        Row, 
        Cell,
        Block, 
        Borders, 
        Paragraph, 
        Table}, 
    layout::{
        Layout, 
        Direction, 
        Constraint}, 
    Frame, 
    text::{
        Span, 
        Text, 
        Spans}, 
    style::{
        Style, 
        Modifier, 
        Color}};
use crossterm::event::{
        self, 
        KeyCode, 
        Event};

enum InputMode {
    Normal,
    Editing,
    Saving,
    Quiting,
    QuitSaving,
    Saved,
    SavedFailed
}


pub struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    data: Vec<Vec<String>>,
    pos: (usize, usize),
    saved: bool,
    filename: Option<String>,
    page_pos:(usize, usize) 
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            data: Vec::new(),
            pos: (0,0),
            saved: true,
            filename: None,
            page_pos: (0,0)
        }
    }
}

impl App {
    pub fn load_file_into_app(filename: String) -> Result<App, io::Error> {
        let mut app = App::default();
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(&filename)?;
        app.filename = Some(filename); 
       
        for row in reader.records() {
            app.data.push(row.unwrap().iter().map(|cell_value| {
                String::from(cell_value)
            }).collect());
            //println!("{:?}", row.unwrap());
        }

        Ok(app)
    }

    pub fn run<B: Backend>(
                mut self,
                terminal: &mut Terminal<B>
                ) -> io::Result<()> {

        loop {
            terminal.draw(|f| self.render_ui(f))?;
            if let Event::Key(key) = event::read()? {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            self.input_mode = InputMode::Editing;
                            match self.data.get(self.pos.0) {
                                Some(row) => {
                                    match row.get(self.pos.1) {
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
                            if self.pos.1 > 0 {
                                self.pos.1 -= 1;
                            }
                        },
                        KeyCode::Right | KeyCode::Char('l') => {
                            self.pos.1 += 1;
                        },
                        KeyCode::Up | KeyCode::Char('k') => {
                            if self.pos.0 > 0 {
                                self.pos.0 -= 1;
                            }
                        },
                        KeyCode::Down | KeyCode::Char('j') => {
                            self.pos.0 += 1;
                        },
                        KeyCode::Char('L') => {
                            self.page_pos.1 += 1;
                        },
                        KeyCode::Char('H') => {
                            if self.page_pos.1 > 0 {
                                self.page_pos.1 -= 1;
                            }
                        },
                        KeyCode::Char('J') => {
                            self.page_pos.0 += 1;
                        },
                        KeyCode::Char('K') => {
                            if self.page_pos.0 > 0 {
                                self.page_pos.0 -= 1;
                            }
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
                    }
                }
            }
        }
    }
   
    fn render_ui<B: Backend>(&self, f: &mut Frame<B>) {
        // Set up top level page structure
        let info_row_height = 1;
        let input_box_height = 3;
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Length(info_row_height),
                    Constraint::Length(input_box_height), 
                    Constraint::Min(0), 
                ].as_ref()) 
            .split(f.size()); 
        let (msg, style) = match self.input_mode { 
            InputMode::Normal => ( 
                vec![ Span::raw("Press "), 
                    Span::styled("q", 
                                 Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to exit, "),
                    Span::styled("e", 
                                 Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to Start editing, "),
                    Span::styled("s", 
                                 Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to save, "),
                    Span::styled("a", 
                                 Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to save as new file.")
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    Span::raw("Press "),
                    Span::styled("Esc", 
                                 Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to stop editing, "),
                    Span::styled("Enter", 
                                 Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to record the message"),
                ],
                Style::default(),
            ),
            InputMode::Saving => (
                vec![
                    Span::raw("Press "),
                    Span::styled("Enter", 
                                 Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to save file as, "),
                    Span::styled("Esc",
                                 Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to cancel saving")
                    
                ],
                Style::default()
            ),
            InputMode::QuitSaving => {
                match self.filename {
                    Some(_) => (
                        vec![
                            Span::raw("Saving. Press any Key to continue")
                        ],
                        Style::default()
                    ),
                    None => (
                        vec![
                            Span::raw("Press "),
                            Span::styled("Enter", 
                                         Style::default().add_modifier(Modifier::BOLD)),
                            Span::raw(" to save file as, "),
                            Span::styled("Esc",
                                         Style::default().add_modifier(Modifier::BOLD)),
                            Span::raw(" to cancel saving")
                            
                        ],
                        Style::default()
                    )
                }
            }
            InputMode::Saved => (
                vec![
                    Span::raw("File saved successfully."),
                    Span::raw("Press any Key to continue")
                ],
                Style::default()
            ),
            InputMode::SavedFailed => (
                vec![
                    Span::raw("File save failed. Try Saving as a new file."),
                    Span::raw("Press any Key to continue")
                ],
                Style::default()
            ),
            InputMode::Quiting => {
                if self.saved == true {
                    (
                        vec![
                            Span::raw("Quiting. Press any Key to continue")
                        ],
                        Style::default()
                    )
                } else {
                    (
                        vec![
                            Span::raw("File not saved."),
                            Span::raw("Save file first? Press "),
                            Span::styled("Y or N", 
                                         Style::default()
                                         .add_modifier(Modifier::BOLD)),
                        ],
                        Style::default()
                    )
                }
            } 
        };
        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let help_message = Paragraph::new(text);
        f.render_widget(help_message, chunks[0]);
        
        let input_title = match self.input_mode {
            InputMode::Normal => "Input - Normal",
            InputMode::SavedFailed => "Input - Saved Failed",
            InputMode::Quiting => "Input - Quiting",
            InputMode::QuitSaving => "Input - Saving and Quiting",
            InputMode::Editing => "Input - Editing",
            InputMode::Saved => "Input - Saved",
            InputMode::Saving => "Input - Saving"
        };
        let input = Paragraph::new(self.input.as_ref())
            .block(Block::default().borders(Borders::ALL).title(input_title));
        f.render_widget(input, chunks[1]);

        // build table
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

        let mut table_rows = Vec::new();
        let mut widths = Vec::new();
        widths.push(Constraint::Length(col_width as u16));
        for col in 1..=cols {
            let max_width = self.get_max_col_width(col-1);
            // print!("{} ", max_width);
            widths.push(Constraint::Length(max_width as u16));
        }
        
        let mut first_row_vec = Vec::new();
        first_row_vec.push(Cell::from(""));
        for col in 0..cols {
            first_row_vec.push(Cell::from(col.to_string()))
        }
        table_rows.push(Row::new(first_row_vec));

        for row in 0..rows {
            let mut row_vec = Vec::new();
            row_vec.push(Cell::from(row.to_string()));
            let default_cell_value = "_____";
            for col in 0..cols {
                let mut cell_has_value = false;
                let mut cell_value = String::from(match self.data.get(row) {
                    Some(data_row) => {
                        match data_row.get(col) {
                            Some(data_cell) => {
                                if data_cell.len() > 0 {
                                    cell_has_value = true;
                                    data_cell
                                } else {
                                    default_cell_value
                                }
                            },
                            None => default_cell_value
                        }
                    },
                    None => default_cell_value
                });
                
                let max_col_width = self.get_max_col_width(col);
                if  cell_value.len() < max_col_width {
                    let diff = max_col_width - cell_value.len();
                    for _ in 0..diff {
                        cell_value.push('_');
                    }
                }

                match self.input_mode {
                    InputMode::Normal => {
                        if self.pos.0 == row && self.pos.1 == col {
                            let style = Style::default()
                                .add_modifier(Modifier::RAPID_BLINK)
                                .fg(Color::Yellow);
                            let cell = Cell::from(
                                Span::styled(cell_value, style)
                            );
                            row_vec.push(cell);
                        } else {
                            let style = if cell_has_value {
                                Style::default()
                            } else {
                                Style::default().fg(Color::DarkGray)
                            };
                            let cell = Cell::from(
                                Span::styled(cell_value, style)
                            );
                            row_vec.push(cell);
                        }
                    }
                    InputMode::Editing => {
                        if self.pos.0 == row && self.pos.1 == col {
                            let style = Style::default().fg(Color::Yellow);
                            let cell = Cell::from(
                                Span::styled(cell_value, style)
                            );
                            row_vec.push(cell);
                        } else {
                            let style = if cell_has_value {
                                Style::default()
                            } else {
                                Style::default().fg(Color::DarkGray)
                            };
                            let cell = Cell::from(
                                Span::styled(cell_value, style)
                            );
                            row_vec.push(cell);
                        }
                    },
                    InputMode::Saving | 
                        InputMode::Saved | 
                        InputMode::SavedFailed |
                        InputMode::Quiting |
                        InputMode::QuitSaving => {
                        let cell = Cell::from(cell_value);
                        row_vec.push(cell);
                    }
                }
            }
            table_rows.push(Row::new(row_vec));
        }
        let table_name = match &self.filename {
            Some(name) => String::from(name),
            None => String::from("Table"),
        };
        let table = Table::new(table_rows)
            .block(Block::default().title(table_name).borders(Borders::ALL))
            .widths(&widths)
            .column_spacing(1)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));
        f.render_widget(table,chunks[2]);
        
        // position cursor
        match self.input_mode {
            InputMode::Normal | 
                InputMode::Saved | 
                InputMode::SavedFailed |
                InputMode::Quiting => {},
            InputMode::Editing | InputMode::Saving | InputMode::QuitSaving => {
                f.set_cursor(
                    chunks[1].x + self.input.len() as u16 + 1, 
                    chunks[1].y + 1
                )
            }
        }
    }

    fn add_value_to_cell(&mut self, input: String) {

        let row: &mut Vec<String> = match self.data.get_mut(self.pos.0) {
            Some(data_row) => data_row,
            None => {
                for _ in 0..=self.pos.0 {
                    self.data.push(Vec::new());
                }
                self.data.get_mut(self.pos.0).unwrap()
            }
        };
        let cell = match row.get_mut(self.pos.1) {
            Some(cell_data) => cell_data,
            None => {
                for _ in 0..=self.pos.1 {
                    row.push(String::new());
                }
                row.get_mut(self.pos.1).unwrap()
            }
        };
        *cell = input;
    }

    fn get_max_col_width(&self, col: usize) -> usize {
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

}


