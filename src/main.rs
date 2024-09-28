use std::{env, io};
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

use model::utils_model::RunningMode;

use view::default_view::render_ui as default_render;
use view::debug_view::render_ui as debug_render;
use controller::default_controller::run;
use utils::handle_args::handle_input_args;
//use controller::debugController::run as run_debug;

mod view;
mod model;
mod controller;
mod utils;

fn main() -> Result<(), io::Error>{
    let args: Vec<String> = env::args().collect();
    // process args
    let (mut app_data, mut app_state, running_mode) = match handle_input_args(args) {
        Ok(res) => res,
        Err(error) => {
            panic!("{:?}", error);
        }
    };

    match running_mode {
        RunningMode::Help => {
            return Ok(());
        }
        _ => {}
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend  = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = match running_mode {
        RunningMode::Normal => run(&mut app_data, 
                                   &mut app_state, 
                                   default_render, 
                                   &mut terminal),
        RunningMode::Debug => run(&mut app_data, 
                                  &mut app_state, 
                                  debug_render, 
                                  &mut terminal),
        RunningMode::Help => Err(io::Error::new(io::ErrorKind::Other, 
                                                "RunningMode Help not valid run option"))
    };

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

