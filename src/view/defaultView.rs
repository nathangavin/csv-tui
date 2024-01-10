use tui::{
    backend::Backend, 
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

use crate::{
    controller::defaultController::{
        InputMode, 
        RunningMode
    }, 
    model::defaultAppModel::Position};

pub fn render_ui<B: Backend>(data: &Vec<Vec<String>>,
                            current_input: &str,
                            input_mode: &InputMode,
                            running_mode: &RunningMode,
                            filename: &Option<String>,
                            is_saved: bool,
                            current_pos: &Position,
                            current_page_pos: &Position,
                            max_widths: Vec<usize>,
                            page_width: usize,
                            page_height: usize,
                            f: &mut Frame<B>) {
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
    let (msg, style) = match input_mode { 
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
            match filename {
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
            match is_saved {
                true => (
                    vec![
                        Span::raw("Quiting. Press any Key to continue")
                    ],
                    Style::default()
                ),
                false => (
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
        },
        InputMode::SelectingRow => (
            vec![
                Span::raw("Press "),
                Span::styled("i", 
                             Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to insert row, "),
                Span::styled("r",
                             Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to remove row, "),
                Span::styled("Esc",
                             Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to cancel"),
            ],
            Style::default()
        ),
        InputMode::SelectingCol => (
            vec![
                Span::raw("Press "),
                Span::styled("i", 
                             Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to insert column, "),
                Span::styled("r",
                             Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to remove column, "),
                Span::styled("Esc",
                             Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to cancel"),
            ],
            Style::default()
        )
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);
    
    let input_title = match input_mode {
        InputMode::Normal => "Input - Normal",
        InputMode::SavedFailed => "Input - Saved Failed",
        InputMode::Quiting => "Input - Quiting",
        InputMode::QuitSaving => "Input - Saving and Quiting",
        InputMode::Editing => "Input - Editing",
        InputMode::Saved => "Input - Saved",
        InputMode::Saving => "Input - Saving",
        InputMode::SelectingRow => "Input - Row Selected",
        InputMode::SelectingCol => "Input - Column Selected"
    };
    let input = Paragraph::new(current_input)
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
    for width in max_widths.as_slice() {
        widths.push(Constraint::Length(*width as u16));
    }
    /*
    for col in 1..=cols {
        let max_width = app.get_max_col_width(col-1);
        widths.push(Constraint::Length(max_width as u16));
    }
    */
    
    let mut first_row_vec = Vec::new();
    first_row_vec.push(Cell::from(""));
    for col in 0..cols {
        let num = (current_page_pos.col() * cols) + col;
        first_row_vec.push(Cell::from(num.to_string()))
    }
    table_rows.push(Row::new(first_row_vec));

    for row in 0..rows {
        let mut row_vec = Vec::new();
        let row_num = (current_page_pos.row() * rows) + row;
        row_vec.push(Cell::from(row_num.to_string()));
        let default_cell_value = "_____";
        for col in 0..cols {
            let col_num = (current_page_pos.col() * cols) + col;
            let mut cell_has_value = false;
            let mut cell_value = String::from(match data.get(row_num) {
                Some(data_row) => {
                    match data_row.get(col_num) {
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
            
            let max_col_width = max_widths.get(col).unwrap();
            if  cell_value.len() < *max_col_width {
                let diff = max_col_width - cell_value.len();
                for _ in 0..diff {
                    cell_value.push('_');
                }
            }

            match input_mode {
                InputMode::Normal => {
                    if current_pos.row() == row && current_pos.col() == col {
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
                    if current_pos.row() == row && current_pos.col() == col {
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
                },
                InputMode::SelectingCol => {
                    if current_pos.col() == col {
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
                InputMode::SelectingRow => {
                    if current_pos.row() == row {
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
                }
            }
        }
        table_rows.push(Row::new(row_vec));
    }
    let current_size_string = format!("Rows - {}, Cols - {}", 
                                        data.len(),
                                        data_width);
    let table_name = match filename {
        Some(name) => String::from(name),
        None => String::from("Table"),
    } + " - " + &current_size_string;

    let table = Table::new(table_rows)
        .block(Block::default().title(table_name).borders(Borders::ALL))
        .widths(&widths)
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    let debug_str = format!("{:?}", data);
    let debug_display = Paragraph::new(debug_str);
    match running_mode {
        RunningMode::Normal => {
            f.render_widget(table,chunks[2]);
        },
        RunningMode::Debug => {
            f.render_widget(debug_display, chunks[2]);
        }
    }
    
    // position cursor
    match input_mode {
        InputMode::Normal | 
            InputMode::Saved | 
            InputMode::SavedFailed |
            InputMode::Quiting |
            InputMode::SelectingCol |
            InputMode::SelectingRow => {},
        InputMode::Editing | InputMode::Saving | InputMode::QuitSaving => {
            f.set_cursor(
                chunks[1].x + current_input.len() as u16 + 1, 
                chunks[1].y + 1
            )
        }
    }
}
