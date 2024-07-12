use std::io;
use tui::{
    backend::Backend,
    Terminal
};

use crate::model::{
    AppStateModel::AppStateModel,
    CsvModel::CsvModel,
    UtilsModel::{
        InputMode,
        Size
    }
};

use crate::view::debugView::render_ui;

pub fn run<B: Backend>(
            app_data: &mut CsvModel,
            app_state: &mut AppStateModel,
            terminal: &mut Terminal<B>,
            ) -> io::Result<()> {

    loop {
        // calculate any data that is needed for the view
        todo!();

        terminal.draw(|f| {
            render_ui(data_slice, 
                      grid_size, 
                      data_size, 
                      column_widths, 
                      corner_pos, 
                      relative_pos, 
                      input_mode, 
                      running_mode, 
                      current_input, 
                      filename, 
                      is_saved, 
                      f)
        })?;

        if let Event::Key(key) = event::read()? {
            todo!();
            // handle input behaviour here
        }
    }
}
