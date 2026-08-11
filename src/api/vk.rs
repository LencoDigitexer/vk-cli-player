use crate::models::VideoItem;
use serde::Deserialize;
use std::collections::HashSet;

const API: &str = "https://api.vk.com/method";

#[derive(Deserialize)]
struct ApiError {
    error_msg: String,
    error_code: i32,
}

#[derive(Deserialize)]
struct Body {
    items: Vec<VideoItem>,
}

#[derive(Deserialize)]
struct ApiResponse {
    response: Option<Body>,
    error: Option<ApiError>,
}

fn call(url: String) -> Result<Vec<VideoItem>, String> {
    let body = reqwest::blocking::get(&url)
        .map_err(|e| format!("сеть: {}", e))?
        .text()
        .map_err(|e| format!("чтение: {}", e))?;

    // Отладка: смотрим в терминал, что ответил VK
    println!("VK API → {}", &body[..body.len().min(300)]);

    let parsed: ApiResponse = serde_json::from_str(&body)
        .map_err(|e| format!("парсинг: {} — {}", e, &body[..body.len().min(200)]))?;

    if let Some(err) = parsed.error {
        return Err(format!("VK API {}: {}", err.error_code, err.error_msg));
    }
    Ok(parsed.response.map(|r| r.items).unwrap_or_default())
}

/// Главная лента: дайджест по популярным категориям (video.getCatalog удалён из API)
pub fn catalog(token: &str) -> Result<Vec<VideoItem>, String> {
    let mut all: Vec<VideoItem> = Vec::new();
    for q in [
        "шоу",
        "технологии",
        "музыка",
        "путешествия",
        "авто",
        "интервью",
    ] {
        if let Ok(mut items) = search(token, q) {
            all.append(&mut items);
        }
    }

    let mut seen = HashSet::new();
    all.retain(|v| seen.insert(v.key()));

    if all.is_empty() {
        return Err("Не удалось загрузить ленту".to_string());
    }
    Ok(all)
}

/// Поиск
pub fn search(token: &str, query: &str) -> Result<Vec<VideoItem>, String> {
    call(format!(
        "{}/video.search?q={}&access_token={}&v=5.199&count=20&hd=1",
        API,
        urlencoding::encode(query),
        token
    ))
}
