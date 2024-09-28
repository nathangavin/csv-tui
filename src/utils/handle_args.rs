use std::collections::HashMap;

use crate::model::{
    csv_model::{
        CsvModel, 
        CsvDelimiter},
    app_state_model::AppStateModel, 
    utils_model::RunningMode};

pub fn handle_input_args(args: Vec<String>) 
    -> Result<(CsvModel, AppStateModel, RunningMode), &'static str> {

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
    let app_data: CsvModel;
    let mut filename: Option<&String> = None;
    let mut delimiter: Option<&CsvDelimiter> = None;
    let mut running_mode = RunningMode::Normal;

    println!("{:?}", args);

    /*
     * not sure why I had this
    if num_args > 4 {
        args.truncate(4);
    }
    */

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

    Ok((app_data, app_state, running_mode))
}
