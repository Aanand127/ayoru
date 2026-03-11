#[test]
fn rejects_missing_query() {
    let err = ani::args::parse_from(["ani"]);
    assert!(err.is_err());
}

#[test]
fn parses_tui_subcommand_without_query() {
    let args = ani::args::parse_from(["ani", "tui"]).unwrap();
    assert!(matches!(args.command, ani::args::Command::Tui));
}

#[test]
fn parses_query_mode_unchanged() {
    let args = ani::args::parse_from(["ani", "frieren"]).unwrap();

    match args.command {
        ani::args::Command::Play { query } => assert_eq!(query, vec!["frieren"]),
        other => panic!("expected play command, got {other:?}"),
    }
}

#[test]
fn rejects_extra_args_after_tui_subcommand() {
    let err = ani::args::parse_from(["ani", "tui", "frieren"]);
    assert!(err.is_err());
}
