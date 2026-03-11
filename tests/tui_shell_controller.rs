use ayoru::app::{PlayerRuntime, ProviderRuntime};
use ayoru::core::models::{Episode, StreamCandidate, Title};
use ayoru::player::detect::{DetectError, Player};
use ayoru::tui::action::Action;
use ayoru::tui::controller::TuiController;
use ayoru::tui::library::{LibraryState, SavedTitle, SavedWatch};
use ayoru::tui::storage::LibraryStorage;

struct SearchProvider;

#[async_trait::async_trait]
impl ProviderRuntime for SearchProvider {
    async fn search(&self, _query: &str) -> Result<Vec<Title>, String> {
        Ok(vec![Title {
            id: "show-1".into(),
            name: "Frieren".into(),
        }])
    }

    async fn episodes(&self, _title_id: &str) -> Result<Vec<Episode>, String> {
        Ok(vec![])
    }

    async fn streams(
        &self,
        _title_id: &str,
        _episode: u32,
        _prefer_sub: bool,
    ) -> Result<Vec<StreamCandidate>, String> {
        Ok(vec![])
    }
}

struct NoopPlayerRuntime;

#[async_trait::async_trait]
impl PlayerRuntime for NoopPlayerRuntime {
    fn detect(&self) -> Result<Player, DetectError> {
        Ok(Player::Mpv)
    }

    async fn launch_and_confirm(
        &self,
        _player: Player,
        _stream_url: &str,
        _title: &str,
        _episode: u32,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }
}

#[tokio::test]
async fn controller_loads_saved_library_state_on_startup() {
    let dir = tempfile::tempdir().unwrap();
    let storage = LibraryStorage::new(dir.path().join("library.json"));
    let seeded = LibraryState {
        favorites: vec![],
        history: vec![],
        recently_watched: vec![SavedWatch {
            title: SavedTitle {
                id: "show-1".into(),
                name: "Frieren".into(),
            },
            episode: 2,
            watched_at: 1,
        }],
    };
    storage.save(&seeded).unwrap();

    let app = TuiController::with_storage(SearchProvider, NoopPlayerRuntime, storage)
        .await
        .unwrap();

    assert_eq!(app.library().recently_watched.len(), 1);
}

#[tokio::test]
async fn toggling_favorite_updates_library_and_persists() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("library.json");
    let storage = LibraryStorage::new(path.clone());
    let mut app = TuiController::with_storage(SearchProvider, NoopPlayerRuntime, storage.clone())
        .await
        .unwrap();

    app.dispatch(Action::InsertChar('f')).await.unwrap();
    app.dispatch(Action::SubmitSearch).await.unwrap();
    app.dispatch(Action::ToggleFavorite).await.unwrap();

    assert_eq!(app.library().favorites.len(), 1);

    let loaded = storage.load().unwrap();
    assert_eq!(loaded.favorites.len(), 1);
}
