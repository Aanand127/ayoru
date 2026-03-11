use ani::app::{PlayerRuntime, ProviderRuntime};
use ani::core::models::{Episode, StreamCandidate, Title};
use ani::player::detect::{DetectError, Player};
use ani::tui::action::Action;
use ani::tui::controller::TuiController;
use ani::tui::state::Mode;

struct EpisodeProvider;

#[async_trait::async_trait]
impl ProviderRuntime for EpisodeProvider {
    async fn search(&self, _query: &str) -> Result<Vec<Title>, String> {
        Ok(vec![Title {
            id: "show-1".into(),
            name: "Frieren".into(),
        }])
    }

    async fn episodes(&self, _title_id: &str) -> Result<Vec<Episode>, String> {
        Ok(vec![
            Episode { number: 1 },
            Episode { number: 2 },
            Episode { number: 3 },
        ])
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
async fn opening_episodes_preserves_search_query_when_navigating_back() {
    let mut app = TuiController::new(EpisodeProvider, NoopPlayerRuntime);

    for ch in "frieren".chars() {
        app.dispatch(Action::InsertChar(ch)).await.unwrap();
    }
    app.dispatch(Action::SubmitSearch).await.unwrap();
    app.dispatch(Action::OpenSelectedTitle).await.unwrap();

    assert_eq!(app.state().mode, Mode::Episodes);
    assert_eq!(app.state().episodes.len(), 3);

    app.dispatch(Action::Back).await.unwrap();

    assert_eq!(app.state().mode, Mode::Search);
    assert_eq!(app.state().query, "frieren");
    assert_eq!(app.state().selected_result, 0);
}
