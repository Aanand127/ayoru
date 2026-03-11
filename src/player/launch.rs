use crate::player::detect::Player;
use std::process::{Command, Stdio};

const DEFAULT_REFERER: &str = "https://allmanga.to";

pub fn spawn_player(player: Player, url: &str, title: &str) -> std::io::Result<()> {
    let spec = command_spec(player, url, title);
    let mut cmd = Command::new(spec.program);
    cmd.args(spec.args);

    cmd.stdout(Stdio::null()).stderr(Stdio::null()).spawn()?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct LaunchSpec {
    pub program: &'static str,
    pub args: Vec<String>,
}

pub fn command_spec(player: Player, url: &str, title: &str) -> LaunchSpec {
    match player {
        Player::Mpv => LaunchSpec {
            program: "mpv",
            args: vec![
                "--force-media-title".to_string(),
                title.to_string(),
                "--referrer".to_string(),
                DEFAULT_REFERER.to_string(),
                url.to_string(),
            ],
        },
        Player::Iina => LaunchSpec {
            program: "iina",
            args: vec![
                format!("--mpv-force-media-title={title}"),
                format!("--mpv-referrer={DEFAULT_REFERER}"),
                url.to_string(),
            ],
        },
        Player::Vlc => LaunchSpec {
            program: "vlc",
            args: vec![
                "--play-and-exit".to_string(),
                format!("--http-referrer={DEFAULT_REFERER}"),
                url.to_string(),
            ],
        },
    }
}
