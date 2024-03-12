use std::{io, env, hash::Hash};
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

use model::{
    CsvModel::{
        CsvModel, 
        CsvDelimiter},
    AppStateModel::AppStateModel, 
    UtilsModel::RunningMode};
use controller::defaultController::run as run;

mod view;
mod model;
mod controller;

fn main() -> Result<(), io::Error>{
    let args: Vec<String> = env::args().collect();
    let mut app_data: CsvModel;
    if args.len() > 1 {
        let filename = match args.get(1) {
            Some(name) => name,
            None => {
                println!("you must provide a CSV filename as an input arg.");
                return Ok(());
            }
        };
        app_data = match CsvModel::load_file(String::from(filename)) {
            Ok(app) => app,
            Err(_) => {
                disable_raw_mode()?;
                println!("Unable to load csv");
                return Ok(());
            }
        };
    } else {
        app_data = CsvModel::default();
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
    let mut app_state: AppStateModel;
    app_state = AppStateModel::from_running_mode(&running_mode);
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend  = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = run(&mut app_data, &mut app_state, &mut terminal, running_mode);

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

fn handle_input_args(mut args: Vec<String>) {
    /*
     * number of args equals different scenarios
     * -f or --filename filename
     * default is comma separated
     * -c or --comma
     * -t or --tab
     * -sc or --semicolon
     * -s or --space
     * -d or --debug
     */

    /*
     * ignore every argument except the above
     */
    
    let num_args = args.len();

    if num_args == 0 {
        // default start, no file opening.
        let delimiter = CsvDelimiter::Comma;
    }

    if num_args > 4 {
        args.truncate(4);
    }

    for (index,arg) in args.iter().enumerate() {
        match arg.as_str() {
            "-f"|"--filename" => {
                 

            },
            "-c"|"--comma" => {},
            "-t"|"--tab" => {},
            "-sc"|"--semicolon" => {},
            "-s"|"--space" => {},
            "-d"|"--debug" => {},
            _ => {}
        }
    } 



}
