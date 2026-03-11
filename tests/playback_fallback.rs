use ayoru::core::models::StreamCandidate;
use ayoru::core::playback::{PlaybackError, attempt_with_fallback};
use std::time::Duration;

#[tokio::test]
async fn fails_candidate_if_playback_not_started_within_6s() {
    let streams = vec![StreamCandidate {
        provider: "wixmp".to_string(),
        url: "u1".to_string(),
        is_sub: true,
        resolution: None,
    }];

    let err = attempt_with_fallback(&streams, Duration::from_millis(20), |_| async {
        tokio::time::sleep(Duration::from_millis(40)).await;
        Ok::<(), std::io::Error>(())
    })
    .await
    .unwrap_err();

    assert!(matches!(err, PlaybackError::AllFailed));
}

#[tokio::test]
async fn tries_each_provider_once_then_fails() {
    let streams = vec![
        StreamCandidate {
            provider: "wixmp".into(),
            url: "u1".into(),
            is_sub: true,
            resolution: None,
        },
        StreamCandidate {
            provider: "wixmp".into(),
            url: "u2".into(),
            is_sub: true,
            resolution: None,
        },
        StreamCandidate {
            provider: "youtube".into(),
            url: "u3".into(),
            is_sub: true,
            resolution: None,
        },
    ];

    let mut seen = Vec::new();
    let err = attempt_with_fallback(&streams, Duration::from_millis(10), |s| {
        seen.push(s.provider.clone());
        async { Err::<(), std::io::Error>(std::io::Error::other("fail")) }
    })
    .await
    .unwrap_err();

    assert!(matches!(err, PlaybackError::AllFailed));
    assert_eq!(seen, vec!["wixmp", "youtube"]);
}
