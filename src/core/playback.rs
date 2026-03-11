use crate::core::models::StreamCandidate;
use std::collections::HashSet;
use std::future::Future;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaybackError {
    AllFailed,
}

pub async fn attempt_with_fallback<F, Fut>(
    ranked_streams: &[StreamCandidate],
    timeout: Duration,
    mut launch: F,
) -> Result<(), PlaybackError>
where
    F: FnMut(&StreamCandidate) -> Fut,
    Fut: Future<Output = Result<(), std::io::Error>>,
{
    let mut attempted = HashSet::new();

    for stream in ranked_streams {
        if !attempted.insert(stream.provider.clone()) {
            continue;
        }

        let result = tokio::time::timeout(timeout, launch(stream)).await;
        if matches!(result, Ok(Ok(()))) {
            return Ok(());
        }
    }

    Err(PlaybackError::AllFailed)
}
