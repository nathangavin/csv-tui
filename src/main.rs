use std::{io, vec, fs::{self, File}};
use crossterm::{
    terminal::{
        enable_raw_mode, 
        EnterAlternateScreen, 
        disable_raw_mode, 
        LeaveAlternateScreen}, 
    execute, 
    event::{
        EnableMouseCapture, 
        DisableMouseCapture, 
        self, 
        KeyCode, 
        Event}};
use tui::{
    backend::{
        CrosstermBackend, 
        Backend}, 
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


enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    data: Vec<Vec<String>>,
    pos: (usize, usize)
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            data: Vec::new(),
            pos: (0,0)
        }
    }
}

fn main() -> Result<(), io::Error>{
    enable_raw_mode()?;
    /*let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(t)
            }
        }
    }); */

    //load_file_into_app("./output.csv".to_string()).unwrap();
    //return Ok(());
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend  = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    /* terminal.draw(|f| {
        ui::<CrosstermBackend<io::Stdout>>(f);
    })?; */
    
    //let app = App::default();
    let app = match load_file_into_app(String::from("output.csv")) {
        Ok(app) => app,
        Err(_) => {
            disable_raw_mode()?;
            println!("Unable to load csv");
            panic!();
        }
    };
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
        )?;
    terminal.show_cursor()?;
    
    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}

fn run_app<B: Backend>(
                terminal: &mut Terminal<B>, 
                mut app: App
                ) -> io::Result<()> {

    loop {
        terminal.draw(|f| ui(f, &app))?;
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                        match app.data.get(app.pos.0) {
                            Some(row) => {
                                match row.get(app.pos.1) {
                                    Some(cell) => {
                                        app.input.push_str(cell)
                                    },
                                    None => {}
                                }
                            },
                            None => {} 
                        }
                    },
                    KeyCode::Char('q') => {
                        let _ = save_data_to_file(app);
                        return Ok(());
                    },
                    KeyCode::Left => {
                        if app.pos.1 > 0 {
                            app.pos.1 -= 1;
                        }
                    },
                    KeyCode::Right => {
                        app.pos.1 += 1;
                    },
                    KeyCode::Up => {
                        if app.pos.0 > 0 {
                            app.pos.0 -= 1;
                        }
                    },
                    KeyCode::Down => {
                        app.pos.0 += 1;
                    },
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        //app.messages.push(app.input.drain(..).collect());
                        let current_input = app.input.drain(..).collect();
                        app = add_value_to_cell(app, current_input);
                        app.input_mode = InputMode::Normal;
                    },
                    KeyCode::Char(char) => {
                        app.input.push(char);
                        //app = add_char_to_cell(app, char);
                    },
                    KeyCode::Backspace => {
                        app.input.pop();
                    },
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    },
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
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
    let (msg, style) = match app.input_mode { 
        InputMode::Normal => ( 
            vec![ Span::raw("Press "), 
                Span::styled("q", 
                             Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", 
                             Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to Start editing."),
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
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);
    
    let input = Paragraph::new(app.input.as_ref())
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
        let max_width = get_max_col_width(app, col-1);
        // print!("{} ", max_width);
        widths.push(Constraint::Length(max_width as u16));
    }

    for row in 0..rows {
        let mut row_vec = Vec::new();
        row_vec.push(Cell::from(row.to_string()));
        for col in 0..cols {
            let mut cell_value = String::from(match app.data.get(row) {
                Some(data_row) => {
                    match data_row.get(col) {
                        Some(data_cell) => {
                            if data_cell.len() > 0 {
                                data_cell
                            } else {
                                "_______________"
                            }
                        },
                        None => {
                            "_______________"
                        }
                    }
                },
                None => {
                    "_______________"
                }
            });
            
            if  cell_value.len() < get_max_col_width(app, col) {
                let diff = get_max_col_width(app, col) - cell_value.len();
                for _ in 0..diff {
                    cell_value.push('_');
                }
            }

            match app.input_mode {
                InputMode::Normal => {
                    if app.pos.0 == row && app.pos.1 == col {
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
                    if app.pos.0 == row && app.pos.1 == col {
                        let style = Style::default().fg(Color::Yellow);
                        let cell = Cell::from(Span::styled(cell_value, style));
                        row_vec.push(cell);
                    } else {
                        let cell = Cell::from(cell_value);
                        row_vec.push(cell);
                    }
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
    match app.input_mode {
        InputMode::Normal => {},
        InputMode::Editing => {
            f.set_cursor(
                chunks[1].x + app.input.len() as u16 + 1, 
                chunks[1].y + 1
            )

        }
    }
}

fn load_file_into_app(filename: String) -> Result<App, io::Error> {
    let mut app = App::default();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;
   
   
    for row in reader.records() {
        app.data.push(row.unwrap().iter().map(|cell_value| {
            String::from(cell_value)
        }).collect());
        //println!("{:?}", row.unwrap());
    }

    Ok(app)
}

fn get_max_col_width(app: &App, col: usize) -> usize {
    let mut max_width = 5;

    for row in app.data.iter() {
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

fn add_value_to_cell(mut app: App, input: String) -> App {

    let row: &mut Vec<String> = match app.data.get_mut(app.pos.0) {
        Some(data_row) => data_row,
        None => {
            for _ in 0..=app.pos.0 {
                app.data.push(Vec::new());
            }
            app.data.get_mut(app.pos.0).unwrap()
        }
    };
    let cell = match row.get_mut(app.pos.1) {
        Some(cell_data) => cell_data,
        None => {
            for _ in 0..=app.pos.1 {
                row.push(String::new());
            }
            row.get_mut(app.pos.1).unwrap()
        }
    };
    *cell = input;

    app
}
fn create_csv_string(app: App) -> String {
    let lengths: Vec<usize> = app.data.iter().map(|row| row.len()).collect();
    let num_cols = match lengths.iter().max() {
        Some(value) => *value,
        None => 0
    };
    let output = app.data.iter().fold(String::new(), |mut sum, row| {
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

fn save_data_to_file(app: App) -> std::io::Result<()>  {
    fs::write("./output.csv", create_csv_string(app))?;
    Ok(())
}

// TODO
// add commands for inserting/deleting rows & columns
// add save as feature
// create input args
