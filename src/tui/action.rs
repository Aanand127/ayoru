#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    InsertChar(char),
    SubmitSearch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    SearchTitles(String),
}
