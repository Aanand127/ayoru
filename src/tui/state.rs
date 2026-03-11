use crate::tui::action::{Action, Effect};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    Search,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TuiState {
    pub mode: Mode,
    pub query: String,
    pub is_loading: bool,
}

impl TuiState {
    pub fn apply(&mut self, action: Action) -> Option<Effect> {
        match action {
            Action::InsertChar(ch) => {
                self.query.push(ch);
                None
            }
            Action::SubmitSearch => {
                self.mode = Mode::Search;
                self.is_loading = true;
                Some(Effect::SearchTitles(self.query.clone()))
            }
        }
    }
}
