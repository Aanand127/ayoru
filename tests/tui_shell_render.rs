use ayoru::core::models::Title;
use ayoru::tui::library::{SavedTitle, SavedWatch};
use ayoru::tui::state::{Panel, TuiState};
use ayoru::tui::ui::render_to_buffer;

fn buffer_contains(buffer: &ratatui::buffer::Buffer, needle: &str) -> bool {
    let content = buffer
        .content
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>();
    content.contains(needle)
}

fn shell_fixture_state() -> TuiState {
    TuiState {
        focused_panel: Panel::Main,
        search_focused: false,
        query: "frieren".into(),
        results: vec![Title {
            id: "show-1".into(),
            name: "Frieren".into(),
        }],
        favorites: vec![SavedTitle {
            id: "show-1".into(),
            name: "Frieren".into(),
        }],
        recently_watched: vec![SavedWatch {
            title: SavedTitle {
                id: "show-1".into(),
                name: "Frieren".into(),
            },
            episode: 3,
            watched_at: 1,
        }],
        history: vec![SavedWatch {
            title: SavedTitle {
                id: "show-2".into(),
                name: "Dungeon Meshi".into(),
            },
            episode: 8,
            watched_at: 1,
        }],
        ..Default::default()
    }
}

#[test]
fn shell_render_shows_search_header_and_context_sections() {
    let buffer = render_to_buffer(&shell_fixture_state(), 120, 32);

    assert!(buffer_contains(&buffer, "AYORU"));
    assert!(buffer_contains(&buffer, "Recently watched"));
    assert!(buffer_contains(&buffer, "Favorites"));
    assert!(buffer_contains(&buffer, "History"));
    assert!(buffer_contains(&buffer, "Frieren"));
    assert!(buffer_contains(&buffer, "Dungeon Meshi"));
}
