use ayoru::app::{PlayerRuntime, ProviderRuntime};
use ayoru::core::models::{Episode, StreamCandidate, Title};
use ayoru::player::detect::{DetectError, Player};
use ayoru::tui::action::Action;
use ayoru::tui::controller::TuiController;

struct SearchProvider {
    titles: Vec<Title>,
}

impl SearchProvider {
    fn with_titles(names: &[&str]) -> Self {
        Self {
            titles: names
                .iter()
                .enumerate()
                .map(|(idx, name)| Title {
                    id: format!("title-{idx}"),
                    name: (*name).to_string(),
                })
                .collect(),
        }
    }
}

#[async_trait::async_trait]
impl ProviderRuntime for SearchProvider {
    async fn search(&self, _query: &str) -> Result<Vec<Title>, String> {
        Ok(self.titles.clone())
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
async fn search_success_populates_results_and_selects_first_item() {
    let provider = SearchProvider::with_titles(&["Frieren", "Fullmetal Alchemist"]);
    let mut app = TuiController::new(provider, NoopPlayerRuntime);

    for ch in "frie".chars() {
        app.dispatch(Action::InsertChar(ch)).await.unwrap();
    }
    app.dispatch(Action::SubmitSearch).await.unwrap();

    assert_eq!(app.state().results.len(), 2);
    assert_eq!(app.state().results[0].name, "Frieren");
    assert_eq!(app.state().selected_result, 0);
    assert!(!app.state().is_loading);
}
