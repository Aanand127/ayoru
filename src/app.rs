use crate::core::models::{Episode, StreamCandidate, Title};
use crate::core::playback::{attempt_with_fallback, PlaybackError};
use crate::core::stream_ranker::rank_streams;
use crate::errors::AppError;
use crate::player::detect::{detect_player, DetectError, Player};
use crate::player::launch::spawn_player;
use std::time::Duration;

#[async_trait::async_trait]
pub trait ProviderRuntime {
    async fn search(&self, query: &str) -> Result<Vec<Title>, String>;
    async fn episodes(&self, title_id: &str) -> Result<Vec<Episode>, String>;
    async fn streams(
        &self,
        title_id: &str,
        episode: u32,
        prefer_sub: bool,
    ) -> Result<Vec<StreamCandidate>, String>;
}

#[async_trait::async_trait]
pub trait PlayerRuntime {
    fn detect(&self) -> Result<Player, DetectError>;
    async fn launch_and_confirm(
        &self,
        player: Player,
        stream_url: &str,
        title: &str,
        episode: u32,
    ) -> Result<(), std::io::Error>;
}

pub struct SystemPlayerRuntime;

#[async_trait::async_trait]
impl PlayerRuntime for SystemPlayerRuntime {
    fn detect(&self) -> Result<Player, DetectError> {
        detect_player()
    }

    async fn launch_and_confirm(
        &self,
        player: Player,
        stream_url: &str,
        title: &str,
        episode: u32,
    ) -> Result<(), std::io::Error> {
        let media_title = format!("{title} Episode {episode}");
        spawn_player(player, stream_url, &media_title)
    }
}

pub async fn run_with<P, R>(query: &str, provider: &P, runtime: &R) -> Result<(), AppError>
where
    P: ProviderRuntime + Sync,
    R: PlayerRuntime + Sync,
{
    if query.trim().is_empty() {
        return Err(AppError::NoQuery);
    }

    let titles = provider.search(query).await.map_err(AppError::Provider)?;
    if titles.is_empty() {
        return Err(AppError::NoResults(query.to_string()));
    }

    // MVP temporary default until terminal picker UI is wired to runtime event loop.
    let title = &titles[0];
    let mut episodes = provider
        .episodes(&title.id)
        .await
        .map_err(AppError::Provider)?;
    episodes.sort_by_key(|e| e.number);
    let episode = episodes
        .first()
        .ok_or_else(|| AppError::Provider("No episodes available".to_string()))?;

    let mut streams = provider
        .streams(&title.id, episode.number, true)
        .await
        .map_err(AppError::Provider)?;
    if streams.is_empty() {
        return Err(AppError::NoPlayableStreams);
    }
    rank_streams(&mut streams);

    let player = runtime
        .detect()
        .map_err(|e| AppError::NoSupportedPlayer(e.to_string()))?;

    let title_name = title.name.clone();
    let episode_no = episode.number;
    attempt_with_fallback(&streams, Duration::from_secs(6), |stream| {
        let url = stream.url.clone();
        let title_name = title_name.clone();
        async move {
            runtime
                .launch_and_confirm(player, &url, &title_name, episode_no)
                .await
        }
    })
    .await
    .map_err(|e| match e {
        PlaybackError::AllFailed => AppError::PlaybackFailed,
    })
}
