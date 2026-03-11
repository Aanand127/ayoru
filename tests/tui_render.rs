use ani::core::models::Title;
use ani::tui::state::{Mode, TuiState};
use ani::tui::ui::render_to_buffer;
use ratatui::style::Color;

fn buffer_contains(buffer: &ratatui::buffer::Buffer, needle: &str) -> bool {
    let content = buffer
        .content
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>();
    content.contains(needle)
}

fn first_fg_for_symbol(buffer: &ratatui::buffer::Buffer, needle: &str) -> Option<Color> {
    buffer
        .content
        .iter()
        .find(|cell| cell.symbol() == needle)
        .map(|cell| cell.fg)
}

#[test]
fn render_search_screen_shows_query_results_and_key_hints() {
    let state = TuiState {
        mode: Mode::Search,
        search_focused: false,
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

    assert!(buffer_contains(&buffer, "AYORU"));
    assert!(buffer_contains(&buffer, "A quieter way to watch anime."));
    assert!(buffer_contains(&buffer, "frieren"));
    assert!(buffer_contains(&buffer, "Frieren"));
    assert!(buffer_contains(&buffer, "Enter"));
    assert!(buffer_contains(&buffer, "Ready"));
    assert_eq!(
        first_fg_for_symbol(&buffer, "A"),
        Some(Color::Rgb(231, 232, 234))
    );
    assert_eq!(
        first_fg_for_symbol(&buffer, ">"),
        Some(Color::Rgb(94, 116, 143))
    );
}

#[test]
fn render_empty_search_screen_uses_branded_copy() {
    let buffer = render_to_buffer(&TuiState::default(), 80, 24);

    assert!(buffer_contains(&buffer, "AYORU"));
    assert!(buffer_contains(&buffer, "A quieter way to watch anime."));
    assert!(buffer_contains(&buffer, "Search, choose, watch."));
    assert!(buffer_contains(&buffer, "Type a title, then press Enter"));
}
