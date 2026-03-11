use crate::app::ProviderRuntime;
use crate::core::models::{Episode, StreamCandidate, Title};
use serde_json::Value;

pub const ALLANIME_REFERER: &str = "https://allmanga.to";
pub const ALLANIME_BASE: &str = "allanime.day";
pub const ALLANIME_API: &str = "https://api.allanime.day";

pub struct AllAnimeProvider;

#[async_trait::async_trait]
impl ProviderRuntime for AllAnimeProvider {
    async fn search(&self, _query: &str) -> Result<Vec<Title>, String> {
        Err("live AllAnime HTTP integration is not yet wired".to_string())
    }

    async fn episodes(&self, _title_id: &str) -> Result<Vec<Episode>, String> {
        Err("live AllAnime HTTP integration is not yet wired".to_string())
    }

    async fn streams(
        &self,
        _title_id: &str,
        _episode: u32,
        _prefer_sub: bool,
    ) -> Result<Vec<StreamCandidate>, String> {
        Err("live AllAnime HTTP integration is not yet wired".to_string())
    }
}

pub fn parse_search_titles(raw: &str) -> Result<Vec<Title>, String> {
    let v: Value = serde_json::from_str(raw).map_err(|e| e.to_string())?;
    let edges = v["data"]["shows"]["edges"]
        .as_array()
        .ok_or("missing edges")?;

    let mut out = Vec::with_capacity(edges.len());
    for edge in edges {
        let id = edge["_id"].as_str().ok_or("missing id")?.to_string();
        let name = edge["name"].as_str().ok_or("missing name")?.to_string();
        out.push(Title { id, name });
    }
    Ok(out)
}

pub fn parse_episodes(raw: &str) -> Result<Vec<Episode>, String> {
    let v: Value = serde_json::from_str(raw).map_err(|e| e.to_string())?;
    let eps = v["data"]["show"]["availableEpisodesDetail"]["sub"]
        .as_array()
        .ok_or("missing episodes")?;

    let mut out: Vec<Episode> = eps
        .iter()
        .filter_map(|e| e.as_str())
        .filter_map(|s| s.parse::<u32>().ok())
        .map(|number| Episode { number })
        .collect();
    out.sort_by_key(|e| e.number);
    out.dedup_by_key(|e| e.number);
    Ok(out)
}

pub fn parse_stream_candidates(
    raw: &str,
    prefer_sub: bool,
) -> Result<Vec<StreamCandidate>, String> {
    let v: Value = serde_json::from_str(raw).map_err(|e| e.to_string())?;
    let sources = v["sources"].as_array().ok_or("missing sources")?;

    let mut out = Vec::new();
    for s in sources {
        let name = s["sourceName"].as_str().ok_or("missing sourceName")?;
        let url = s["sourceUrl"]
            .as_str()
            .ok_or("missing sourceUrl")?
            .to_string();
        let provider = match name {
            "Default" => "wixmp",
            "Yt-mp4" => "youtube",
            "S-mp4" => "sharepoint",
            "Luf-Mp4" => "hianime",
            _ => continue,
        }
        .to_string();

        out.push(StreamCandidate {
            provider,
            url,
            is_sub: prefer_sub,
            resolution: None,
        });
    }
    Ok(out)
}
