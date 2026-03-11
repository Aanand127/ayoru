use ayoru::tui::action::Action;
use ayoru::tui::runtime::{InputCommand, map_key_code_for_state};
use ayoru::tui::state::{Mode, Panel, TuiState};
use crossterm::event::KeyCode;

#[test]
fn slash_and_text_input_focus_search_and_append_query() {
    assert_eq!(
        map_key_code_for_state(&TuiState::default(), KeyCode::Char('/')),
        Some(InputCommand::FocusSearch)
    );
    assert_eq!(
        map_key_code_for_state(&TuiState::default(), KeyCode::Char('f')),
        Some(InputCommand::Action(Action::InsertChar('f')))
    );
}

#[test]
fn navigation_keys_map_to_selection_actions() {
    let state = TuiState {
        mode: Mode::Search,
        focused_panel: Panel::Main,
        search_focused: false,
        ..Default::default()
    };

    assert_eq!(
        map_key_code_for_state(&state, KeyCode::Down),
        Some(InputCommand::Action(Action::MoveDown))
    );
    assert_eq!(
        map_key_code_for_state(&state, KeyCode::Char('k')),
        Some(InputCommand::Action(Action::MoveUp))
    );
}

#[test]
fn search_focus_treats_j_and_k_as_query_text() {
    let state = TuiState::default();

    assert_eq!(
        map_key_code_for_state(&state, KeyCode::Char('j')),
        Some(InputCommand::Action(Action::InsertChar('j')))
    );
    assert_eq!(
        map_key_code_for_state(&state, KeyCode::Char('k')),
        Some(InputCommand::Action(Action::InsertChar('k')))
    );
}

#[test]
fn tab_moves_between_shell_panels() {
    let state = TuiState::default();

    assert_eq!(
        map_key_code_for_state(&state, KeyCode::Tab),
        Some(InputCommand::Action(Action::FocusNextPanel))
    );
}

#[test]
fn f_toggles_favorite_when_search_is_not_focused() {
    let state = TuiState {
        focused_panel: Panel::Main,
        search_focused: false,
        ..Default::default()
    };

    assert_eq!(
        map_key_code_for_state(&state, KeyCode::Char('f')),
        Some(InputCommand::Action(Action::ToggleFavorite))
    );
}
