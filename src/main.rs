use inquire::{Select, Text};
use reqwest::Client;
use serde::Deserialize;
use std::fs;
use std::process::Command;
use std::process::Stdio;
use urlencoding::encode;

#[derive(Deserialize, Debug)]
struct VkApiResponse {
    response: Option<ResponseData>,
    error: Option<VkError>,
}

#[derive(Deserialize, Debug)]
struct VkError {
    error_msg: String,
    error_code: i32,
}

#[derive(Deserialize, Debug)]
struct ResponseData {
    items: Vec<VideoItem>,
}

#[derive(Deserialize, Debug, Clone)]
struct VideoItem {
    owner_id: i64,
    id: i64,
    title: String,
    duration: Option<i32>,
}

impl std::fmt::Display for VideoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dur = self.duration.unwrap_or(0);
        let mins = dur / 60;
        let secs = dur % 60;
        write!(f, "{} ({}:{:02})", self.title, mins, secs)
    }
}

#[tokio::main]
async fn main() {
    println!("=== VK Video CLI Player (Rust) ===");

    let token = match fs::read_to_string(".vk_token") {
        Ok(t) => {
            let token = t.trim().to_string();
            if token.is_empty() {
                println!("❌ Файл .vk_token пустой");
                return;
            }
            println!("✅ Использую токен из .vk_token");
            token
        }
        Err(_) => {
            println!("❌ Файл .vk_token не найден.");
            println!("ℹ️  Создайте его командой:");
            println!("   echo 'ВАШ_ТОКЕН' > .vk_token");
            println!("ℹ️  Получить токен можно через https://vkhost.github.io/");
            return;
        }
    };

    let query = Text::new("Что будем искать?")
        .prompt()
        .expect("Ошибка ввода запроса");

    println!("⏳ Поиск видео на серверах ВКонтакте...");

    let encoded_query = encode(&query);
    let url = format!(
        "https://api.vk.com/method/video.search?q={}&access_token={}&v=5.199&count=20&hd=1",
        encoded_query, token
    );

    let client = Client::new();
    let res = client.get(&url).send().await;

    match res {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();

            if !status.is_success() {
                println!("❌ Ошибка HTTP {}: {}", status, body);
                return;
            }

            match serde_json::from_str::<VkApiResponse>(&body) {
                Ok(api_res) => {
                    if let Some(err) = api_res.error {
                        println!(
                            "❌ Ошибка API ВК (Код {}): {}",
                            err.error_code, err.error_msg
                        );
                        if err.error_code == 28 {
                            println!("ℹ️  Токен не имеет прав для video.search");
                            println!(
                                "ℹ️  Получите новый токен с правами video через vkhost.github.io"
                            );
                        } else if err.error_code == 5 {
                            println!("ℹ️  Токен истек или невалиден");
                            println!("ℹ️  Удалите .vk_token и получите новый токен");
                        }
                        return;
                    }

                    if let Some(data) = api_res.response {
                        if data.items.is_empty() {
                            println!("😔 Ничего не найдено по запросу '{}'.", query);
                            return;
                        }

                        let selected = Select::new(
                            "🔽 Выберите видео для просмотра (стрелки + Enter):",
                            data.items,
                        )
                        .prompt();

                        match selected {
                            Ok(video) => {
                                let vk_url = format!(
                                    "https://vkvideo.ru/video{}_{}",
                                    video.owner_id, video.id
                                );
                                play_with_mpv(&vk_url);
                            }
                            Err(_) => println!("Выбор отменен."),
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Ошибка парсинга JSON: {}", e);
                    println!("Сырой ответ:\n{}", &body[..body.len().min(500)]);
                }
            }
        }
        Err(e) => println!("❌ Ошибка сети: {}", e),
    }
}

fn play_with_mpv(url: &str) {
    println!("🔄 Передача потока в MPV...");

    let mut yt_dlp = match Command::new("yt-dlp")
        .args(["-o", "-", "-q", "--no-warnings", url])
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(cmd) => cmd,
        Err(_) => {
            println!("❌ Ошибка: не удалось запустить yt-dlp.");
            return;
        }
    };

    let stdout = yt_dlp
        .stdout
        .take()
        .expect("Не удалось получить stdout от yt-dlp");

    let mpv = Command::new("mpv")
        .arg("-") // Читать из stdin
        .arg("--title=VK Video")
        .stdin(stdout)
        .spawn();

    match mpv {
        Ok(mut mpv_proc) => {
            println!("▶️ Запуск MPV...");
            let _ = mpv_proc.wait();
        }
        Err(_) => {
            println!("❌ Ошибка: не удалось запустить mpv. Установите его: sudo apt install mpv");
        }
    }

    let _ = yt_dlp.wait();
}
