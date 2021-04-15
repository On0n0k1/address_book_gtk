// Used to check which is the last unsaved row before button behaviors
pub enum LastEdit{
    NONE,
    NEW(usize),
    UNSAVED(usize),
}

impl Clone for LastEdit{
    fn clone(&self) -> Self {
        match self {
            LastEdit::NONE => {
                LastEdit::NONE
            },
            LastEdit::NEW(idx) => {
                LastEdit::NEW(idx.clone())
            },
            LastEdit::UNSAVED(idx) => {
                LastEdit::UNSAVED(idx.clone())
            }
        }
    }
}