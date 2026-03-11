use ayoru::app::{PlayerRuntime, ProviderRuntime};
use ayoru::core::models::{Episode, StreamCandidate, Title};
use ayoru::player::detect::{DetectError, Player};
use ayoru::tui::action::Action;
use ayoru::tui::controller::TuiController;
use ayoru::tui::state::Mode;

struct PlaybackProvider;

#[async_trait::async_trait]
impl ProviderRuntime for PlaybackProvider {
    async fn search(&self, _query: &str) -> Result<Vec<Title>, String> {
        Ok(vec![Title {
            id: "show-1".into(),
            name: "Frieren".into(),
        }])
    }

    async fn episodes(&self, _title_id: &str) -> Result<Vec<Episode>, String> {
        Ok(vec![Episode { number: 1 }])
    }

    async fn streams(
        &self,
        _title_id: &str,
        _episode: u32,
        _prefer_sub: bool,
    ) -> Result<Vec<StreamCandidate>, String> {
        Ok(vec![StreamCandidate {
            provider: "wixmp".into(),
            url: "https://example.test/stream".into(),
            is_sub: true,
            resolution: Some(1080),
        }])
    }
}

struct FailingPlayerRuntime;

#[async_trait::async_trait]
impl PlayerRuntime for FailingPlayerRuntime {
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
        Err(std::io::Error::other("launch failed"))
    }
}

struct SuccessPlayerRuntime;

#[async_trait::async_trait]
impl PlayerRuntime for SuccessPlayerRuntime {
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
async fn play_episode_returns_to_episodes_with_failure_message() {
    let mut app = TuiController::new(PlaybackProvider, FailingPlayerRuntime);

    for ch in "frieren".chars() {
        app.dispatch(Action::InsertChar(ch)).await.unwrap();
    }
    app.dispatch(Action::SubmitSearch).await.unwrap();
    app.dispatch(Action::OpenSelectedTitle).await.unwrap();
    app.dispatch(Action::PlaySelectedEpisode).await.unwrap();

    assert_eq!(app.state().mode, Mode::Episodes);
    assert_eq!(
        app.state().message.as_deref(),
        Some("Playback failed after trying all providers")
    );
}

#[tokio::test]
async fn successful_playback_records_recent_and_history() {
    let mut app = TuiController::new(PlaybackProvider, SuccessPlayerRuntime);

    for ch in "frieren".chars() {
        app.dispatch(Action::InsertChar(ch)).await.unwrap();
    }
    app.dispatch(Action::SubmitSearch).await.unwrap();
    app.dispatch(Action::OpenSelectedTitle).await.unwrap();
    app.dispatch(Action::PlaySelectedEpisode).await.unwrap();

    assert_eq!(app.library().history.len(), 1);
    assert_eq!(app.library().recently_watched.len(), 1);
}
