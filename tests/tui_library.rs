use ayoru::tui::library::{LibraryState, SavedTitle, SavedWatch};

fn saved_title(id: &str, name: &str) -> SavedTitle {
    SavedTitle {
        id: id.to_string(),
        name: name.to_string(),
    }
}

fn saved_watch(id: &str, name: &str, episode: u32) -> SavedWatch {
    SavedWatch {
        title: saved_title(id, name),
        episode,
        watched_at: 1,
    }
}

#[test]
fn toggling_favorite_is_idempotent() {
    let mut library = LibraryState::default();
    let title = saved_title("show-1", "Frieren");

    library.toggle_favorite(title.clone());
    assert_eq!(library.favorites, vec![title.clone()]);

    library.toggle_favorite(title);

    assert!(library.favorites.is_empty());
}

#[test]
fn recording_watch_updates_history_and_recently_watched() {
    let mut library = LibraryState::default();
    library.record_watch(saved_watch("show-1", "Frieren", 3));

    assert_eq!(library.history.len(), 1);
    assert_eq!(library.recently_watched.len(), 1);
    assert_eq!(library.history[0].episode, 3);
    assert_eq!(library.recently_watched[0].episode, 3);
}
