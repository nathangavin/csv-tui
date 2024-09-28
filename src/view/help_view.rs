pub fn print_help_text() {
    let help_text = "
Usage:
csv-tui [options]       Run CSV editor

Options:
 -h, --help             Print this help message
 -f, --filename         Open file defined in next arg in editor
 -d, --debug            Run CSV editor in Debug mode
 -c, --comma            Set the CSV delimiter to comma
 -t, --tab              Set the CSV delimiter to tab
 -sc, --semicolon       Set the CSV delimiter to semicolon
 -s, --space            Set the CSV delimiter to space

Examples:
 csv-tui                    Opens empty editor
 csv-tui -f test.csv        Opens test.csv into the editor
 csv-tui -f test.csv -sc    Tries to open test.csv using semicolon as delimiter
 csv-tui --tab              Opens empty editor, setting delimiter to tab
 csv-tui --debug            Opens empty editor in debug mode
";
    println!("{}", help_text);
}
