use ayoru::player::detect::Player;
use ayoru::player::launch::command_spec;

#[test]
fn mpv_command_includes_referrer_and_title() {
    let spec = command_spec(Player::Mpv, "https://example/stream", "Show Episode 1");
    assert_eq!(spec.program, "mpv");
    assert!(spec.args.contains(&"--referrer".to_string()));
    assert!(spec.args.contains(&"https://allmanga.to".to_string()));
    assert!(spec.args.contains(&"--force-media-title".to_string()));
}

#[test]
fn iina_command_includes_mpv_referrer_flag() {
    let spec = command_spec(Player::Iina, "https://example/stream", "Show Episode 1");
    assert_eq!(spec.program, "iina");
    assert!(
        spec.args
            .iter()
            .any(|a| a == "--mpv-referrer=https://allmanga.to")
    );
}

#[test]
fn vlc_command_includes_http_referrer_flag() {
    let spec = command_spec(Player::Vlc, "https://example/stream", "Show Episode 1");
    assert_eq!(spec.program, "vlc");
    assert!(
        spec.args
            .iter()
            .any(|a| a == "--http-referrer=https://allmanga.to")
    );
}
