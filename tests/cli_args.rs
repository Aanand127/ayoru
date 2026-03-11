#[test]
fn rejects_missing_query() {
    let err = ani::args::parse_from(["ani"]);
    assert!(err.is_err());
}
