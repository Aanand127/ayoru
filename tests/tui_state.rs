use ani::tui::action::{Action, Effect};
use ani::tui::state::{Mode, TuiState};

#[test]
fn submit_search_sets_loading_state() {
    let mut state = TuiState {
        query: "frieren".into(),
        ..Default::default()
    };

    let effect = state.apply(Action::SubmitSearch);

    assert_eq!(state.mode, Mode::Search);
    assert!(state.is_loading);
    assert_eq!(effect, Some(Effect::SearchTitles("frieren".into())));
}

#[test]
fn typing_updates_query_without_side_effects() {
    let mut state = TuiState::default();

    let effect = state.apply(Action::InsertChar('f'));

    assert_eq!(state.query, "f");
    assert_eq!(effect, None);
}
