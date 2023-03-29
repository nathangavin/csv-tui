use std::{
    io, 
    vec};
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
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend  = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    /* terminal.draw(|f| {
        ui::<CrosstermBackend<io::Stdout>>(f);
    })?; */
    
    let app = App::default();
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
                    },
                    KeyCode::Char('q') => {
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
                        todo!();
                    },
                    KeyCode::Char(char) => {
                        app.input.push(char);
                        app = add_char_to_cell(app, char);
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
                    Constraint::Length(3),
                    Constraint::Min(0),
                ].as_ref()
            )
        .split(f.size()); 

    // set up dynamic message at top level
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
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
            if cell_value.len() < get_max_col_width(app, col) {
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
    f.render_widget(table,chunks[1]);

    match app.input_mode {
        InputMode::Normal => {},
        InputMode::Editing => {
            let prev_cells_width_sum = match app.data.get(app.pos.0) {
                Some(row) => {
                    let row_sum: usize = row[0..app.pos.1].iter().map(|cell| {
                        if cell.len() > 0 {
                            return cell.len() + 1;
                        }
                        return col_width + 1;
                    }).sum();
                    row_sum + col_width + 1
                },
                None => {
                    (col_width + 1) * (app.pos.1 + 1)
                } 
            };
            //print!("{:?}", prev_cells_width_sum);
            let x = chunks[1].x + 
                 prev_cells_width_sum as u16 +
                get_cell_value_len(app, app.pos.0, app.pos.1) as u16 + 1;
            let y = chunks[1].y + app.pos.0 as u16 + 1;
            f.set_cursor(x,y)
        }
    }
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

fn get_cell_value_len(app: &App, row: usize, col: usize) -> usize {
    match app.data.get(row) {
        Some(row) => {
            match row.get(col) {
                Some(cell_value) => cell_value.len(),
                None => 0
            }
        },
        None => 0
    }
}

fn add_char_to_cell(mut app: App, char: char) -> App {
    let row: &mut Vec<String> = match app.data.get_mut(app.pos.0) {
        Some(data_row) => data_row,
        None => {
            for _ in 0..app.pos.0 + 1 {
                app.data.push(Vec::new());
            }
            app.data.get_mut(app.pos.0).unwrap()
        }
    };
    let cell = match row.get_mut(app.pos.1) {
        Some(cell_data) => cell_data,
        None => {
            for _ in 0..app.pos.1 + 1 {
                row.push(String::new());
            }
            row.get_mut(app.pos.1).unwrap()
        }
    };
    cell.push(char);

    app
}
