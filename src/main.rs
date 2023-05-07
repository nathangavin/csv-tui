use std::{io, env};
use tui::{
    Terminal,
    backend::CrosstermBackend
};
use crossterm::{
    terminal::{
        enable_raw_mode, 
        EnterAlternateScreen, 
        disable_raw_mode, 
        LeaveAlternateScreen}, 
    execute, 
    event::{
        EnableMouseCapture, 
        DisableMouseCapture}};

mod app;
use app::RunningMode;
use app::App;

fn main() -> Result<(), io::Error>{
    let args: Vec<String> = env::args().collect();
    let app: App;
    if args.len() > 1 {
        let filename = match args.get(1) {
            Some(name) => name,
            None => {
                println!("you must provide a CSV filename as an input arg.");
                return Ok(());
            }
        };
        app = match App::load_file_into_app(String::from(filename)) {
            Ok(app) => app,
            Err(_) => {
                disable_raw_mode()?;
                println!("Unable to load csv");
                return Ok(());
            }
        };
    } else {
        app = App::default();
    }
    let running_mode = match args.get(2) {
        Some(flag) => {
            match flag.as_str() {
                "-debug" => {RunningMode::Debug},
                _ => {RunningMode::Normal}
            }
        },
        None => {
            RunningMode::Normal
        }
    };
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend  = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = app.run(&mut terminal, running_mode);

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

// TODO
// add button for showing commands
// split app model code and app view code, in order to add debugging viewmode
