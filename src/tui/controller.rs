use crate::app::{PlayerRuntime, ProviderRuntime};
use crate::core::playback::{PlaybackError, attempt_with_fallback};
use crate::core::stream_ranker::rank_streams;
use crate::errors::AppError;
use crate::tui::action::{Action, Effect};
use crate::tui::state::TuiState;
use std::time::Duration;

pub struct TuiController<P, R> {
    provider: P,
    player: R,
    state: TuiState,
}

impl<P, R> TuiController<P, R>
where
    P: ProviderRuntime,
    R: PlayerRuntime,
{
    pub fn new(provider: P, player: R) -> Self {
        Self {
            provider,
            player,
            state: TuiState::default(),
        }
    }

    pub fn state(&self) -> &TuiState {
        &self.state
    }

    pub async fn dispatch(&mut self, action: Action) -> Result<(), AppError> {
        let effect = self.state.apply(action);

        match effect {
            Some(Effect::SearchTitles(query)) => match self.provider.search(&query).await {
                Ok(results) => {
                    self.state.apply(Action::SearchCompleted(results));
                    Ok(())
                }
                Err(err) => {
                    self.state.apply(Action::SearchFailed(err));
                    Ok(())
                }
            },
            Some(Effect::LoadEpisodes(title)) => match self.provider.episodes(&title.id).await {
                Ok(episodes) => {
                    self.state
                        .apply(Action::EpisodesCompleted { title, episodes });
                    Ok(())
                }
                Err(err) => {
                    self.state.apply(Action::EpisodesFailed(err));
                    Ok(())
                }
            },
            Some(Effect::PlayEpisode { title, episode }) => {
                let mut streams = self
                    .provider
                    .streams(&title.id, episode.number, true)
                    .await
                    .map_err(AppError::Provider)?;
                if streams.is_empty() {
                    self.state.apply(Action::PlaybackFailed(
                        "No playable streams found".to_string(),
                    ));
                    return Ok(());
                }

                rank_streams(&mut streams);

                let player = match self.player.detect() {
                    Ok(player) => player,
                    Err(err) => {
                        self.state.apply(Action::PlaybackFailed(err.to_string()));
                        return Ok(());
                    }
                };

                let title_name = title.name.clone();
                let episode_number = episode.number;
                let player_runtime = &self.player;
                let playback_result =
                    attempt_with_fallback(&streams, Duration::from_secs(6), |stream| {
                        let url = stream.url.clone();
                        let title_name = title_name.clone();
                        async move {
                            player_runtime
                                .launch_and_confirm(player, &url, &title_name, episode_number)
                                .await
                        }
                    })
                    .await;

                match playback_result {
                    Ok(()) => {
                        self.state.apply(Action::PlaybackStarted);
                        Ok(())
                    }
                    Err(PlaybackError::AllFailed) => {
                        self.state.apply(Action::PlaybackFailed(
                            "Playback failed after trying all providers".to_string(),
                        ));
                        Ok(())
                    }
                }
            }
            None => Ok(()),
        }
    }
}
