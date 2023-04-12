use std::{io::{self, Error as IO_Error, ErrorKind}, vec, fs, error::Error};
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

pub enum InputMode {
    Normal,
    Editing,
    Saving
}

pub struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    data: Vec<Vec<String>>,
    pos: (usize, usize),
    saved: bool,
    filename: Option<String>
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            data: Vec::new(),
            pos: (0,0),
            saved: true,
            filename: None 
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
                            // if saved = true, no need to save
                            match &self.filename {
                                Some(_) => {
                                    let _ = self.save_data_to_file();
                                },
                                None => { todo!() }
                            }
                            // only on a successful save should it quit, Other
                            // wise it should should say saved failed and stay
                            // in normal mode
                            return Ok(());
                        },
                        KeyCode::Char('s') => {
                            // if saved = true, no need to save
                            match &self.filename {
                                Some(_) => {
                                    let _ = self.save_data_to_file();
                                },
                                None => { todo!() }
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
                        KeyCode::Right  | KeyCode::Char('l') => {
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
                            //app = add_char_to_cell(app, char);
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
                                Ok(_) => self.saved = true,
                                Err(_) => {
                                    self.saved = false; 
                                    todo!();
                                }
                            }
                            self.input_mode = InputMode::Normal;
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
    
    
    fn render_ui<B: Backend>(&self, f: &mut Frame<B>) {
        // Set up top level page structure
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3), 
                    Constraint::Min(0), 
                ].as_ref()) 
            .split(f.size()); 
                //set up dynamic message at top level 
        let (msg, style) = match self.input_mode { 
            InputMode::Normal => ( 
                vec![ Span::raw("Press "), 
                    Span::styled("q", 
                                 Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to save and exit, "),
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
            )
        };
        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let help_message = Paragraph::new(text);
        f.render_widget(help_message, chunks[0]);
        
        let input = Paragraph::new(self.input.as_ref())
            .block(Block::default().borders(Borders::ALL).title("Input"));
        f.render_widget(input, chunks[1]);

        // build table
        let rows : usize = 50;
        let cols : usize = 50;
        let col_width: usize = 5;
        let mut table_rows = Vec::new();

        let mut widths = Vec::new();
        widths.push(Constraint::Length(col_width as u16));
        for col in 1..=cols {
            let max_width = self.get_max_col_width(col-1);
            // print!("{} ", max_width);
            widths.push(Constraint::Length(max_width as u16));
        }

        for row in 0..rows {
            let mut row_vec = Vec::new();
            row_vec.push(Cell::from(row.to_string()));
            let default_cell_value = "_____";
            for col in 0..cols {
                let mut cell_value = String::from(match self.data.get(row) {
                    Some(data_row) => {
                        match data_row.get(col) {
                            Some(data_cell) => {
                                if data_cell.len() > 0 {
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
                            let cell = Cell::from(Span::styled(cell_value, style));
                            row_vec.push(cell);
                        } else {
                            let cell = Cell::from(cell_value);
                            row_vec.push(cell);
                        }
                    }
                    InputMode::Editing => {
                        if self.pos.0 == row && self.pos.1 == col {
                            let style = Style::default().fg(Color::Yellow);
                            let cell = Cell::from(Span::styled(cell_value, style));
                            row_vec.push(cell);
                        } else {
                            let cell = Cell::from(cell_value);
                            row_vec.push(cell);
                        }
                    },
                    InputMode::Saving => {
                        let cell = Cell::from(cell_value);
                        row_vec.push(cell);
                    }
                }
            }
            table_rows.push(Row::new(row_vec));
        }

        let table = Table::new(table_rows)
            .block(Block::default().title("Table").borders(Borders::ALL))
            .widths(&widths)
            .column_spacing(1)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));
        f.render_widget(table,chunks[2]);
        
        // position cursor
        match self.input_mode {
            InputMode::Normal => {},
            InputMode::Editing | InputMode::Saving => {
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


