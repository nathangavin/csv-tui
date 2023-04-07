use std::io;
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
use app::load_file_into_app;
use app::run_app;

/// App holds the state of the application

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

// TODO
// add commands for inserting/deleting rows & columns
// add save as feature
// create input args
