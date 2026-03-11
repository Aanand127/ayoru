use ayoru::tui::library::{LibraryState, SavedTitle, SavedWatch};
use ayoru::tui::storage::LibraryStorage;

fn saved_watch(id: &str, name: &str, episode: u32) -> SavedWatch {
    SavedWatch {
        title: SavedTitle {
            id: id.to_string(),
            name: name.to_string(),
        },
        episode,
        watched_at: 1,
    }
}

#[test]
fn saves_and_loads_library_state_as_json() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("library.json");
    let storage = LibraryStorage::new(path);

    let mut library = LibraryState::default();
    library.record_watch(saved_watch("show-1", "Frieren", 1));

    storage.save(&library).unwrap();
    let loaded = storage.load().unwrap();

    assert_eq!(loaded.history.len(), 1);
    assert_eq!(loaded.recently_watched.len(), 1);
}

#[test]
fn missing_library_file_loads_as_empty_state() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("missing.json");
    let storage = LibraryStorage::new(path);

    let loaded = storage.load().unwrap();

    assert_eq!(loaded, LibraryState::default());
}
