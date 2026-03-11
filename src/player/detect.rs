use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Mpv,
    Iina,
    Vlc,
}

impl Player {
    pub fn bin(self) -> &'static str {
        match self {
            Player::Mpv => "mpv",
            Player::Iina => "iina",
            Player::Vlc => "vlc",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectError {
    NoSupportedPlayer { supported: Vec<&'static str> },
}

impl Display for DetectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DetectError::NoSupportedPlayer { supported } => {
                write!(
                    f,
                    "No supported player found. Install one of: {}",
                    supported.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for DetectError {}

pub fn choose_player_with<F>(exists: F) -> Result<Player, DetectError>
where
    F: Fn(&str) -> bool,
{
    for p in [Player::Mpv, Player::Iina, Player::Vlc] {
        if exists(p.bin()) {
            return Ok(p);
        }
    }

    Err(DetectError::NoSupportedPlayer {
        supported: vec!["mpv", "iina", "vlc"],
    })
}

pub fn detect_player() -> Result<Player, DetectError> {
    choose_player_with(|name| which::which(name).is_ok())
}
