use ani::provider::allanime::{parse_episodes, parse_search_titles, parse_stream_candidates};

#[test]
fn parses_search_results_from_fixture() {
    let raw = std::fs::read_to_string("tests/fixtures/search.json").unwrap();
    let titles = parse_search_titles(&raw).unwrap();
    assert_eq!(titles.len(), 2);
    assert_eq!(titles[0].id, "show-1");
}

#[test]
fn parses_episode_list_and_sorts_ascending() {
    let raw = std::fs::read_to_string("tests/fixtures/episodes.json").unwrap();
    let episodes = parse_episodes(&raw).unwrap();
    assert_eq!(
        episodes.iter().map(|e| e.number).collect::<Vec<_>>(),
        vec![1, 2, 3]
    );
}

#[test]
fn parses_provider_candidates() {
    let raw = std::fs::read_to_string("tests/fixtures/streams.json").unwrap();
    let streams = parse_stream_candidates(&raw, true).unwrap();
    let providers = streams.into_iter().map(|s| s.provider).collect::<Vec<_>>();
    assert_eq!(providers, vec!["wixmp", "youtube", "sharepoint", "hianime"]);
}
