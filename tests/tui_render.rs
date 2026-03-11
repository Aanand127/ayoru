use ani::core::models::Title;
use ani::tui::state::{Mode, TuiState};
use ani::tui::ui::render_to_buffer;

fn buffer_contains(buffer: &ratatui::buffer::Buffer, needle: &str) -> bool {
    let content = buffer
        .content
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>();
    content.contains(needle)
}

#[test]
fn render_search_screen_shows_query_results_and_key_hints() {
    let state = TuiState {
        mode: Mode::Search,
        query: "frieren".into(),
        is_loading: false,
        results: vec![
            Title {
                id: "show-1".into(),
                name: "Frieren".into(),
            },
            Title {
                id: "show-2".into(),
                name: "Fullmetal Alchemist".into(),
            },
        ],
        selected_result: 0,
        current_title: None,
        episodes: vec![],
        selected_episode: 0,
        message: Some("Ready".into()),
    };

    let buffer = render_to_buffer(&state, 80, 24);

    assert!(buffer_contains(&buffer, "Search"));
    assert!(buffer_contains(&buffer, "frieren"));
    assert!(buffer_contains(&buffer, "Frieren"));
    assert!(buffer_contains(&buffer, "Enter"));
}
