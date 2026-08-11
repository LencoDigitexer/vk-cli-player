use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ImagePreview {
    pub url: String,
    pub width: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VideoItem {
    pub owner_id: i64,
    pub id: i64,
    pub title: String,
    pub duration: Option<i64>,
    pub views: Option<i64>,
    pub date: Option<i64>,
    pub image: Option<Vec<ImagePreview>>,
    pub direct_url: Option<String>,
}

impl VideoItem {
    /// Ссылка для воспроизведения/скачивания: direct_url из ответа API, иначе собранная вручную
    pub fn play_url(&self) -> String {
        self.direct_url.clone().unwrap_or_else(|| self.page_url())
    }

    pub fn key(&self) -> (i64, i64) {
        (self.owner_id, self.id)
    }

    pub fn page_url(&self) -> String {
        format!("https://vkvideo.ru/video{}_{}", self.owner_id, self.id)
    }

    /// Превью с шириной, ближайшей к 320px
    pub fn thumb_url(&self) -> Option<String> {
        let imgs = self.image.as_ref()?;
        let best = imgs
            .iter()
            .min_by_key(|i| (i.width.unwrap_or(0) as i32 - 320).abs())?;
        let mut url = best.url.clone();
        if url.starts_with("//") {
            url = format!("https:{}", url);
        }
        Some(url)
    }
}

pub fn format_duration(secs: i64) -> String {
    let (h, m, s) = (secs / 3600, (secs % 3600) / 60, secs % 60);
    if h > 0 {
        format!("{}:{:02}:{:02}", h, m, s)
    } else {
        format!("{}:{:02}", m, s)
    }
}

pub fn format_views(v: i64) -> String {
    if v >= 1_000_000 {
        format!("{:.1} млн", v as f64 / 1_000_000.0)
    } else if v >= 1_000 {
        format!("{:.0} тыс", v as f64 / 1_000.0)
    } else {
        v.to_string()
    }
}

pub fn time_ago_ru(ts: i64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let d = (now - ts).max(0);
    match d {
        0..=59 => "только что".into(),
        60..=3599 => format!("{} минут назад", d / 60),
        3600..=86_399 => format!("{} часов назад", d / 3600),
        86_400..=2_591_999 => format!("{} дней назад", d / 86_400),
        2_592_000..=31_535_999 => format!("{} месяцев назад", d / 2_592_000),
        _ => format!("{} лет назад", d / 31_536_000),
    }
}
