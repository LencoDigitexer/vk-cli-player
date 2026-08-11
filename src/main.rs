use eframe::egui;
use egui::ScrollArea;
use reqwest::Client;
use serde::Deserialize;
use std::fs;
use std::process::{Child, Command};
use std::sync::mpsc;
use std::thread;
use urlencoding::encode;

#[derive(Deserialize, Debug, Clone)]
struct VideoItem {
    owner_id: i64,
    id: i64,
    title: String,
    duration: Option<i32>,
}

#[derive(Deserialize)]
struct VkApiResponse {
    response: Option<ResponseData>,
    error: Option<VkError>,
}

#[derive(Deserialize)]
struct VkError {
    error_msg: String,
    error_code: i32,
}

#[derive(Deserialize)]
struct ResponseData {
    items: Vec<VideoItem>,
}

enum AppState {
    Search,
    Playing { video: VideoItem },
}

struct VkVideoApp {
    token: String,
    search_query: String,
    videos: Vec<VideoItem>,
    state: AppState,
    mpv_child: Option<Child>,
    error_message: Option<String>,
    is_searching: bool,
    clicked_video: Option<VideoItem>,
    clicked_download: Option<VideoItem>,
    search_rx: Option<mpsc::Receiver<Result<Vec<VideoItem>, String>>>,
}

impl VkVideoApp {
    fn new() -> Self {
        let token = fs::read_to_string(".vk_token")
            .unwrap_or_default()
            .trim()
            .to_string();

        Self {
            token,
            search_query: String::new(),
            videos: Vec::new(),
            state: AppState::Search,
            mpv_child: None,
            error_message: None,
            is_searching: false,
            clicked_video: None,
            clicked_download: None,
            search_rx: None,
        }
    }

    fn search_videos(&mut self) {
        if self.search_query.is_empty() {
            return;
        }

        self.is_searching = true;
        self.error_message = None;

        let query = self.search_query.clone();
        let token = self.token.clone();

        let (tx, rx) = mpsc::channel();
        self.search_rx = Some(rx);

        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let result = rt.block_on(Self::search_videos_async(&query, &token));
            let _ = tx.send(result);
        });
    }

    async fn search_videos_async(query: &str, token: &str) -> Result<Vec<VideoItem>, String> {
        let encoded_query = encode(query);
        let url = format!(
            "https://api.vk.com/method/video.search?q={}&access_token={}&v=5.199&count=20&hd=1",
            encoded_query, token
        );

        let client = Client::new();
        let res = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Ошибка сети: {}", e))?;

        let body = res
            .text()
            .await
            .map_err(|e| format!("Ошибка чтения ответа: {}", e))?;

        let api_res: VkApiResponse =
            serde_json::from_str(&body).map_err(|e| format!("Ошибка парсинга JSON: {}", e))?;

        if let Some(err) = api_res.error {
            return Err(format!(
                "Ошибка API ({}): {}",
                err.error_code, err.error_msg
            ));
        }

        if let Some(data) = api_res.response {
            Ok(data.items)
        } else {
            Err("Пустой ответ от API".to_string())
        }
    }

    /// Запускает mpv как внешний процесс. mpv сам вытащит стрим через встроенный yt-dlp.
    fn play_video(&mut self, video: VideoItem) {
        let vk_url = format!("https://vkvideo.ru/video{}_{}", video.owner_id, video.id);

        match Command::new("mpv")
            .arg(&vk_url)
            .arg("--force-window=immediate")
            .arg(format!("--title=VK: {}", video.title))
            .spawn()
        {
            Ok(child) => {
                self.mpv_child = Some(child);
                self.state = AppState::Playing { video };
            }
            Err(e) => {
                self.error_message = Some(format!("Не удалось запустить mpv: {}", e));
            }
        }
    }

    fn download_video(&self, video: &VideoItem) {
        let vk_url = format!("https://vkvideo.ru/video{}_{}", video.owner_id, video.id);

        let download_dir = dirs::download_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        let filename = format!("{}_{}.mp4", video.owner_id, video.id);
        let filepath = download_dir.join(&filename);

        let filepath_clone = filepath.clone();
        let vk_url_clone = vk_url.clone();

        thread::spawn(move || {
            let status = Command::new("yt-dlp")
                .args([
                    "-o",
                    filepath_clone.to_str().unwrap(),
                    "-f",
                    "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best",
                    "--merge-output-format",
                    "mp4",
                    vk_url_clone.as_str(),
                ])
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("✅ Видео сохранено: {}", filepath_clone.display())
                }
                Ok(_) => eprintln!("❌ Ошибка скачивания"),
                Err(e) => eprintln!("❌ Не удалось запустить yt-dlp: {}", e),
            }
        });
    }

    fn stop_video(&mut self) {
        if let Some(mut child) = self.mpv_child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        self.state = AppState::Search;
    }
}

impl eframe::App for VkVideoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Опрашиваем канал поиска
        if let Some(rx) = self.search_rx.take() {
            match rx.try_recv() {
                Ok(Ok(videos)) => {
                    self.videos = videos;
                    self.is_searching = false;
                }
                Ok(Err(e)) => {
                    self.error_message = Some(e);
                    self.is_searching = false;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    self.search_rx = Some(rx);
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.is_searching = false;
                }
            }
        }

        // Если окно mpv закрыли — возвращаемся в режим поиска
        let mpv_finished = matches!(
            self.mpv_child.as_mut().map(|c| c.try_wait()),
            Some(Ok(Some(_)))
        );
        if mpv_finished {
            self.mpv_child = None;
            self.state = AppState::Search;
        }

        if self.is_searching {
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎬 VK Video Player");

            if let Some(ref error) = self.error_message {
                ui.colored_label(egui::Color32::RED, format!("❌ {}", error));
                ui.separator();
            }

            match &self.state {
                AppState::Search => {
                    ui.horizontal(|ui| {
                        ui.label("Поиск:");
                        let response = ui.text_edit_singleline(&mut self.search_query);

                        if ui.button("🔍 Найти").clicked()
                            || (response.lost_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                        {
                            self.search_videos();
                        }
                    });

                    ui.separator();

                    if self.is_searching {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label("Поиск видео...");
                        });
                    }

                    let videos_clone = self.videos.clone();

                    ScrollArea::vertical()
                        .max_height(ui.available_height() - 100.0)
                        .show(ui, |ui| {
                            for video in &videos_clone {
                                ui.horizontal(|ui| {
                                    ui.label("📺");

                                    let mut title = video.title.clone();
                                    if let Some(duration) = video.duration {
                                        let mins = duration / 60;
                                        let secs = duration % 60;
                                        title = format!("{} ({}:{:02})", title, mins, secs);
                                    }

                                    if ui.button(&title).clicked() {
                                        self.clicked_video = Some(video.clone());
                                    }

                                    if ui.button("💾 Скачать").clicked() {
                                        self.clicked_download = Some(video.clone());
                                    }
                                });
                            }
                        });

                    if let Some(video) = self.clicked_video.take() {
                        self.play_video(video);
                    }

                    if let Some(video) = self.clicked_download.take() {
                        self.download_video(&video);
                    }
                }

                AppState::Playing { video } => {
                    ui.label(format!("▶️ Воспроизведение: {}", video.title));
                    ui.label("ℹ️ Видео играет в отдельном окне mpv");

                    if ui.button("⏹ Остановить").clicked() {
                        self.stop_video();
                    }
                }
            }
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "VK Video Player",
        options,
        Box::new(|_cc| Ok(Box::new(VkVideoApp::new()))),
    )?;

    Ok(())
}
