use std::io;
use tui::{
    backend::Backend,
    Terminal
};

use crate::model::{
    AppStateModel::AppStateModel,
    CsvModel::CsvModel,
    UtilsModel::{
        RunningMode,
        InputMode,
        Size
    }
};

pub fn run<B: Backend>(
            app_data: &mut CsvModel,
            app_state: &mut AppStateModel,
            terminal: &mut Terminal<B>,
            ) -> io::Result<()> {


    Ok(())
}
