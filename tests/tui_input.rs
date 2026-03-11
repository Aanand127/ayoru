use ani::tui::action::Action;
use ani::tui::runtime::{InputCommand, map_key_code};
use crossterm::event::KeyCode;

#[test]
fn slash_and_text_input_focus_search_and_append_query() {
    assert_eq!(
        map_key_code(KeyCode::Char('/')),
        Some(InputCommand::FocusSearch)
    );
    assert_eq!(
        map_key_code(KeyCode::Char('f')),
        Some(InputCommand::Action(Action::InsertChar('f')))
    );
}

#[test]
fn navigation_keys_map_to_selection_actions() {
    assert_eq!(
        map_key_code(KeyCode::Down),
        Some(InputCommand::Action(Action::MoveDown))
    );
    assert_eq!(
        map_key_code(KeyCode::Char('k')),
        Some(InputCommand::Action(Action::MoveUp))
    );
}
