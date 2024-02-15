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

pub enum RunningMode {
    Normal,
    Debug
}

#[derive(Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize
}

#[derive(Clone, Copy)]
pub struct Size {
    pub width: usize,
    pub height: usize
}

