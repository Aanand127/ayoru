use ani::app::{PlayerRuntime, ProviderRuntime, run_with};
use ani::core::models::{Episode, StreamCandidate, Title};
use ani::errors::AppError;
use ani::player::detect::{DetectError, Player};

struct NoResultsProvider;

#[async_trait::async_trait]
impl ProviderRuntime for NoResultsProvider {
    async fn search(&self, _query: &str) -> Result<Vec<Title>, String> {
        Ok(vec![])
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

struct GoodProvider;

#[async_trait::async_trait]
impl ProviderRuntime for GoodProvider {
    async fn search(&self, _query: &str) -> Result<Vec<Title>, String> {
        Ok(vec![Title {
            id: "show-1".into(),
            name: "Test Show".into(),
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
            url: "https://example/stream".into(),
            is_sub: true,
            resolution: Some(720),
        }])
    }
}

struct NoPlayerRuntime;

#[async_trait::async_trait]
impl PlayerRuntime for NoPlayerRuntime {
    fn detect(&self) -> Result<Player, DetectError> {
        Err(DetectError::NoSupportedPlayer {
            supported: vec!["mpv", "iina", "vlc"],
        })
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
async fn exits_cleanly_on_no_results() {
    let err = run_with("frieren", &NoResultsProvider, &NoPlayerRuntime)
        .await
        .unwrap_err();
    assert!(matches!(err, AppError::NoResults(_)));
}

#[tokio::test]
async fn exits_with_install_guidance_when_no_player_found() {
    let err = run_with("frieren", &GoodProvider, &NoPlayerRuntime)
        .await
        .unwrap_err();
    assert!(matches!(err, AppError::NoSupportedPlayer(_)));
}
