use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppError {
    NoQuery,
    NoResults(String),
    Provider(String),
    NoSupportedPlayer(String),
    NoPlayableStreams,
    PlaybackFailed,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NoQuery => write!(f, "Query is required"),
            AppError::NoResults(q) => write!(f, "No results found for \"{}\"", q),
            AppError::Provider(e) => write!(f, "Provider error: {e}"),
            AppError::NoSupportedPlayer(e) => write!(f, "{e}"),
            AppError::NoPlayableStreams => write!(f, "No playable streams found"),
            AppError::PlaybackFailed => write!(f, "Playback failed after trying all providers"),
        }
    }
}

impl std::error::Error for AppError {}
