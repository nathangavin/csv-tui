use std::{collections::HashMap, env, io};
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
    let (mut app_data,mut app_state) = match handle_input_args(args) {
        Ok(res) => res,
        Err(error) => {
            panic!("{:?}", error);
        }
    };
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend  = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = run(&mut app_data, &mut app_state, &mut terminal);

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

fn handle_input_args(mut args: Vec<String>) -> Result<(CsvModel, AppStateModel), &'static str> {
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

    let num_args = args.len();
    let mut app_data: CsvModel;
    let mut filename: Option<&String>;
    let mut delimiter: Option<&CsvDelimiter>;
    let mut running_mode = RunningMode::Normal;

    if num_args == 0 {
        // default start, no file opening.
        delimiter = Some(&CsvDelimiter::Comma);
    }

    if num_args > 4 {
        args.truncate(4);
    }

    let delimiters = HashMap::from([
        ("-c", CsvDelimiter::Comma),
        ("-t", CsvDelimiter::Tab),
        ("-sc", CsvDelimiter::Semicolon),
        ("-s", CsvDelimiter::Space)
    ]);

    let long_delimiters = HashMap::from([
        ("--comma", "-c"),
        ("--tab", "-t"),
        ("--semicolon", "-sc"),
        ("--space", "-s")
    ]);

    for (index,arg) in args.iter().enumerate() {
        match arg.as_str() {
            "-f"|"--filename" => {
                filename = args.get(index + 1);
            },
            "-c"|"--comma" |
            "-t"|"--tab" |
            "-sc"|"--semicolon" |
            "-s"|"--space" => {
                let delimiter_err_message = "Error - unable to determine chosen delimiter.";
                delimiter = match args.get(index) {
                    Some(tag) => {
                        match long_delimiters.get(&tag[..]) {
                            Some(short_tag) => delimiters.get(short_tag),
                            None => delimiters.get(&tag[..])
                        }
                    },
                    None => {
                        return Err(delimiter_err_message);
                    }
                }

            },
            "-d"|"--debug" => {
                running_mode = RunningMode::Debug;
            },
            _ => {}
        };
    } 

    match filename {
        Some(fname) => {
            match delimiter {
                Some(delim) => {
                    app_data = match CsvModel::load_file(fname, delim) {
                        Ok(app) => app,
                        Err(_) => {
                            return Err("Error - Unable to load CSV with defined delimiter.");
                        }
                    };
                },
                None => {
                   app_data = match CsvModel::load_file(fname, &CsvDelimiter::Comma) {
                       Ok(app) => app,
                       Err(_) => {
                           return Err("Error - Unable to load CSV");
                       }
                   }

                }
            }
        },
        None => {
            match delimiter {
                Some(delim) => {
                    app_data = match CsvModel::default_with_delimiter(delim) {
                        Ok(app) => app,
                        Err(_) => {
                            return Err("Error - Unable to create new CSV with defined delimiter");
                        }
                    }
                },
                None => {
                    app_data = CsvModel::default();
                    
                }
            }
        }
    }

    let app_state = AppStateModel::from_running_mode(&running_mode);

    Ok((app_data, app_state))
}
