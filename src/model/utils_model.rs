#[derive(Debug, PartialEq, Eq, Hash)]
pub enum InputMode {
    Normal,
    Editing,
    SelectingRow,
    SelectingCol,
    Saving,
    Quiting,
    QuitSaving,
    Saved,
    SavedFailed
}

pub enum _InsertMode {
    Adding,
    Removing
}

#[derive(Debug, PartialEq)]
pub enum RunningMode {
    Normal,
    Debug,
    Help
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize
}

#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub width: usize,
    pub height: usize
}

