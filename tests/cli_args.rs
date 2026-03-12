#[test]
fn parses_bare_command_as_tui() {
    let args = ayoru::args::parse_from(["ayoru"]).unwrap();
    assert!(matches!(args.command, ayoru::args::Command::Tui));
}

#[test]
fn parses_tui_subcommand_without_query() {
    let args = ayoru::args::parse_from(["ayoru", "tui"]).unwrap();
    assert!(matches!(args.command, ayoru::args::Command::Tui));
}

#[test]
fn parses_query_mode_unchanged() {
    let args = ayoru::args::parse_from(["ayoru", "frieren"]).unwrap();

    match args.command {
        ayoru::args::Command::Play { query } => assert_eq!(query, vec!["frieren"]),
        other => panic!("expected play command, got {other:?}"),
    }
}

#[test]
fn rejects_extra_args_after_tui_subcommand() {
    let err = ayoru::args::parse_from(["ayoru", "tui", "frieren"]);
    assert!(err.is_err());
}
