use std::io;

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

use crate::model::utils_model::{
    Position,
    Size,
    InputMode,
    RunningMode
};

/// function renders the UI into the terminal frame provided. 
///
/// # Arguments
///
/// * `data_slice` - a 2D Vec of Strings which represent the slice of the CSV data that
/// fits on the page.
///
/// * `grid_size` - a Size struct which is the size of the data Vec.
///
/// * `data_size` - a Size struct which is the saize of the entire CSV data.
///
/// * `column_widths` - a Vec of usize which describes how wide each column should
/// be in order to fit the data. length should match the width defined in grid_size and 
/// data.
///
/// * `corner_pos` - a Position struct which states where the top left corner of the 
/// current data slice appears in the overall csv data. Used to generate row and col 
/// numbers.
///
/// * `relative_pos` - a Position struct which states the position of the currently 
/// selected cell. The position is relative to the current frame, not the overall position
/// in the csv data.
///
/// * `input_mode` - an InputMode enum which states the current input mode that the application is
/// in.
///
/// * `running_mode` - a RunningMode enum which states the current input mode that the application
/// is in.
///
/// * `current_input` - a str which is the current value of the input field.
///
/// * `filename` - the name of the file being edited.
///
/// * `is_saved` - a boolean desribing if the current state of the file is saved to disk or not.
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
    /*
     * configure chunk structure, defining top level as info box, second
     * as input box, and third as a filler of the rest of the space, to 
     * hold the table.
     */
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

    let (msg, style) = generate_header_msg(input_mode, filename, is_saved);
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);
    
    let input_title = generate_input_title(input_mode);
    let input = Paragraph::new(current_input)
        .block(Block::default().borders(Borders::ALL).title(input_title));
    f.render_widget(input, chunks[1]);

    /*
     * construct the table
     */

    let col_width: usize = 5;
    let mut widths = Vec::new();
    widths.push(Constraint::Length(col_width as u16));
    for col in 0..grid_size.width {
        let width = match column_widths.get(col) {
            Some(w) => w,
            None => &col_width
        };
        widths.push(Constraint::Length(*width as u16));
    }
    let mut table_rows: Vec<Row> = Vec::new();

    let mut first_row_vec = Vec::new();
    first_row_vec.push(Cell::from(""));
    for col in 0..grid_size.width {
        let num = corner_pos.col + col;
        first_row_vec.push(Cell::from(num.to_string()));
    }
    table_rows.push(Row::new(first_row_vec));

    let default_cell_value = "_____";
    for row in 0..grid_size.height {
        let mut row_vec = Vec::new();
        let row_num = corner_pos.row + row;
        row_vec.push(Cell::from(row_num.to_string()));
        for col in 0..grid_size.width {
            let mut cell_has_value = false;
            let mut cell_value = String::from(match data_slice.get(row) {
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
            
            let max_col_width : usize = match column_widths.get(col) {
                Some(length) => *length,
                None => default_cell_value.len()
            };
            if  cell_value.len() < max_col_width {
                let diff = max_col_width - cell_value.len();
                for _ in 0..diff {
                    cell_value.push('_');
                }
            }

            match input_mode {
                InputMode::Normal => {
                    if relative_pos.row == row && relative_pos.col == col {
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
                    if relative_pos.row == row && relative_pos.col == col {
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
                    if relative_pos.col == col {
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
                    if relative_pos.row == row {
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
                                        data_size.height,
                                        data_size.width);
    let table_name = match filename {
        Some(name) => String::from(name),
        None => String::from("Table"),
    } + " - " + &current_size_string;

    let table = Table::new(table_rows)
        .block(Block::default().title(table_name).borders(Borders::ALL))
        .widths(&widths)
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    let debug_str = format!("{:?}", data_slice);
    let debug_display = Paragraph::new(debug_str);
    match running_mode {
        RunningMode::Normal => {
            f.render_widget(table,chunks[2]);
        },
        RunningMode::Debug => {
            f.render_widget(debug_display, chunks[2]);
        },
        RunningMode::Help => {
            return;
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

        InputMode::Editing | 
            InputMode::Saving | 
            InputMode::QuitSaving => {
                f.set_cursor(
                    chunks[1].x + current_input.len() as u16 + 1, 
                    chunks[1].y + 1
                )
        }
    }
}

fn generate_header_msg(input_mode: &InputMode, 
                       filename: &Option<String>, 
                       is_saved: bool) -> (Vec<Span<'static>>, Style) {
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

    return (msg, style);
}

fn generate_input_title(input_mode: &InputMode) -> &str {
    match input_mode {
        InputMode::Normal => "Input - Normal",
        InputMode::SavedFailed => "Input - Saved Failed",
        InputMode::Quiting => "Input - Quiting",
        InputMode::QuitSaving => "Input - Saving and Quiting",
        InputMode::Editing => "Input - Editing",
        InputMode::Saved => "Input - Saved",
        InputMode::Saving => "Input - Saving",
        InputMode::SelectingRow => "Input - Row Selected",
        InputMode::SelectingCol => "Input - Column Selected"
    }
}
