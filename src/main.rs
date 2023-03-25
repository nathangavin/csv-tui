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
    /// History of recorded messages
    messages: Vec<String>,

    data: Vec<Vec<String>>
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            data: Vec::new()
        }
    }
}

/* fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10)
                ].as_ref()
            )
        .split(f.size());

    println!("{:?}", chunks);
    let block = Block::default()
        .title("Block")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
/*    let block2 = Block::default()
        .title("Block 2")
        .borders(Borders::ALL);
    f.render_widget(block2, chunks[0]); */
} */

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

    // thread::sleep(Duration::from_millis(5000));
    
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
                        app.messages.push(app.input.drain(..).collect());
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
    // println!("{:?}", chunks);
    /*let grid_chunks: Vec<Vec<Rect>> = chunks
        .chunks(1)
        .map(|col| {
            //println!("{:?}", col);
            Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(0),
                ].as_ref())
                .split(col[0])
        }).collect(); */
    // println!("{:?}",grid_chunks.len());
    
    /*for column in grid_chunks {
        for cell in column {
            let block = Block::default()
                //.title("Cell")
                .borders(Borders::ALL);
            // f.render_widget(block, cell);
        }
    }*/
    let rows : usize = 50;
    let cols : usize = 50;
    let mut table_rows = Vec::new();
    for _ in 0..rows {
        let mut row_vec = Vec::new();
        for _ in 0..cols {
            row_vec.push(Cell::from("_____"));
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
    /*let chunks = Layout::default()
        .direction(Direction::Vertical)
        // .margin(2)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ].as_ref())
        .split(f.size()); */
    // println!("{:?}", chunks);
    // println!("{:?}", f.size());
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

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages = List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    // f.render_widget(messages,chunks[2]);
}
fn cell_border_style() -> Style {
    Style::default().fg(Color::White).bg(Color::Black).add_modifier(Modifier::BOLD)
}
// https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/
