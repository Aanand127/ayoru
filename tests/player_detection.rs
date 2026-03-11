use ayoru::player::detect::{DetectError, Player, choose_player_with};

#[test]
fn chooses_mpv_then_iina_then_vlc() {
    let chosen = choose_player_with(|name| matches!(name, "iina" | "vlc")).unwrap();
    assert_eq!(chosen, Player::Iina);

    let chosen = choose_player_with(|name| name == "mpv").unwrap();
    assert_eq!(chosen, Player::Mpv);
}

#[test]
fn returns_install_guidance_when_none_found() {
    let err = choose_player_with(|_| false).unwrap_err();
    assert!(matches!(err, DetectError::NoSupportedPlayer { .. }));
    assert!(err.to_string().contains("mpv"));
    assert!(err.to_string().contains("iina"));
    assert!(err.to_string().contains("vlc"));
}
