use std::{io, thread, time::{Duration, Instant}, sync::mpsc, vec};
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, self, KeyEventKind, KeyCode, Event}, style::Stylize};
use tui::{backend::{CrosstermBackend, Backend}, Terminal, widgets::{Row, Cell,Block, Borders, Paragraph, ListItem, List, Table}, layout::{Layout, Direction, Constraint, Rect, Margin}, Frame, text::{Span, Text, Spans}, style::{Style, Modifier, Color}};


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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {

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
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        //app.messages.push(app.input.drain(..).collect());
                        todo!();
                    },
                    KeyCode::Char(char) => {
                        app.input.push(char);
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
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to Start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
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
    let mut table_rows = Vec::new();
    for row in 0..rows {
        let mut row_vec = Vec::new();
        row_vec.push(Cell::from(row.to_string()));
        for col in 0..cols {
            match app.input_mode {
                InputMode::Normal => {
                    let cell_value = "_____";


                    if app.pos.0 == row && app.pos.1 == col {
                        let style = Style::default().add_modifier(Modifier::RAPID_BLINK);
                        let cell = Cell::from(Span::styled(cell_value, style));
                        row_vec.push(cell);
                    } else {
                        let cell = Cell::from(cell_value);
                        row_vec.push(cell);
                    }
                    //row_vec.push(cell);
                }
                InputMode::Editing => {}
            }
            //row_vec.push(Cell::from("_____"));
        }
        table_rows.push(Row::new(row_vec));
    }
    
    let mut widths = Vec::new();
    for _ in 0..cols {
        widths.push(Constraint::Length(5));
    }

    let table = Table::new(table_rows)
        .header(
            Row::new(vec!["top1", "top2", "top3"])
            .style(Style::default().fg(Color::Red))
            .bottom_margin(1)
            )
        .block(Block::default().title("Table").borders(Borders::ALL))
        .widths(&widths)
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(table,chunks[1]);
    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    // f.render_widget(input, chunks[1]);
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
fn cell_border_style() -> Style {
    Style::default().fg(Color::White).bg(Color::Black).add_modifier(Modifier::BOLD)
}
// https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/
