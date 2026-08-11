// history.rs — «Продолжить просмотр»
use crate::models::VideoItem;
use std::fs;

const FILE: &str = ".vk_history.json";

pub fn load() -> Vec<VideoItem> {
    fs::read_to_string(FILE)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn push(video: &VideoItem) -> Vec<VideoItem> {
    let mut h = load();
    h.retain(|v| v.key() != video.key());
    h.insert(0, video.clone());
    h.truncate(12);
    let _ = fs::write(FILE, serde_json::to_string_pretty(&h).unwrap_or_default());
    h
}
