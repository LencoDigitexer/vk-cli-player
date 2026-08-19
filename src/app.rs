use crate::api;
use crate::models::VideoItem;
use crate::services::{downloader, history, player, thumbnails};
use crate::ui;
use eframe::egui;
use std::collections::{HashMap, HashSet};
use std::process::Child;
use std::sync::mpsc;
use std::thread;

type Key = (i64, i64);

pub struct VkVideoApp {
    token: String,
    query: String,
    feed: Vec<VideoItem>,
    feed_label: String,
    chip: String,
    active_nav: String,
    loader: thumbnails::ThumbnailLoader,
    textures: HashMap<Key, egui::TextureHandle>,
    pending: HashSet<Key>,
    failed: HashSet<Key>,
    feed_tx: mpsc::Sender<(u64, Result<Vec<VideoItem>, String>)>,
    feed_rx: Option<mpsc::Receiver<(u64, Result<Vec<VideoItem>, String>)>>,
    feed_gen: u64,
    loading: bool,
    error: Option<String>,
    mpv: Option<Child>,
    history: Vec<VideoItem>,
}

impl VkVideoApp {
    pub fn new() -> Self {
        let token = std::fs::read_to_string(".vk_token")
            .unwrap_or_default()
            .trim()
            .to_string();
        let (feed_tx, feed_rx) = mpsc::channel();

        let mut app = Self {
            token,
            query: String::new(),
            feed: Vec::new(),
            feed_label: "Главная".into(),
            chip: "Все".into(),
            active_nav: "Главная".into(),
            loader: thumbnails::ThumbnailLoader::new(),
            textures: HashMap::new(),
            pending: HashSet::new(),
            failed: HashSet::new(),
            feed_tx,
            feed_rx: Some(feed_rx),
            feed_gen: 0,
            loading: false,
            error: None,
            mpv: None,
            history: history::load(),
        };
        app.load_home();
        app
    }

    fn load_home(&mut self) {
        self.loading = true;
        self.error = None;
        self.feed_gen += 1;
        let gen = self.feed_gen;
        let token = self.token.clone();
        let tx = self.feed_tx.clone();
        thread::spawn(move || {
            let _ = tx.send((gen, api::vk::catalog(&token)));
        });
    }

    fn load_search(&mut self, q: &str) {
        self.loading = true;
        self.error = None;
        self.feed_gen += 1;
        let gen = self.feed_gen;
        let token = self.token.clone();
        let q = q.to_string();
        let tx = self.feed_tx.clone();
        thread::spawn(move || {
            let _ = tx.send((gen, api::vk::search(&token, &q)));
        });
    }

    fn request_thumbs(&mut self) {
        let all: Vec<VideoItem> = self
            .feed
            .iter()
            .chain(self.history.iter())
            .cloned()
            .collect();
        for v in all {
            let k = v.key();
            if self.textures.contains_key(&k)
                || self.pending.contains(&k)
                || self.failed.contains(&k)
            {
                continue;
            }
            if let Some(url) = v.thumb_url() {
                self.pending.insert(k);
                self.loader.request(k, url);
            }
        }
    }

    fn play(&mut self, video: VideoItem) {
        let url = video.play_url();
        println!("▶️ Воспроизведение: {}", url);
        match player::play(&video.play_url(), &video.title) {
            Ok(child) => {
                self.mpv = Some(child);
                self.history = history::push(&video);
            }
            Err(e) => self.error = Some(format!("mpv: {}", e)),
        }
    }

    fn handle_action(&mut self, action: Option<ui::feed::CardAction>, video: &VideoItem) {
        match action {
            Some(ui::feed::CardAction::Play) => self.play(video.clone()),
            Some(ui::feed::CardAction::Download) => downloader::start(&video.play_url()),
            None => {}
        }
    }
}

impl eframe::App for VkVideoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ── фоновые данные ──────────────────────────
        for (k, res) in self.loader.drain() {
            self.pending.remove(&k);
            match res {
                Ok(ci) => {
                    let t = ctx.load_texture(
                        format!("thumb_{}_{}", k.0, k.1),
                        ci,
                        egui::TextureOptions::LINEAR,
                    );
                    self.textures.insert(k, t);
                }
                Err(()) => {
                    self.failed.insert(k);
                }
            }
        }

        if let Some(rx) = self.feed_rx.take() {
            loop {
                match rx.try_recv() {
                    // Применяем только ответ актуального запроса
                    Ok((gen, res)) if gen == self.feed_gen => match res {
                        Ok(videos) => {
                            self.feed = videos;
                            self.loading = false;
                            self.request_thumbs();
                        }
                        Err(e) => {
                            self.error = Some(e);
                            self.loading = false;
                        }
                    },
                    // Устаревший ответ (гонка) — выбрасываем
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
            self.feed_rx = Some(rx);
        }

        if matches!(self.mpv.as_mut().map(|c| c.try_wait()), Some(Ok(Some(_)))) {
            self.mpv = None;
        }
        if self.loading {
            ctx.request_repaint();
        }

        // ── шапка ───────────────────────────────────
        let submitted = egui::TopBottomPanel::top("topbar")
            .exact_height(60.0)
            .frame(ui::theme::panel_frame(10.0))
            .show(ctx, |ui| ui::topbar::show(ui, &mut self.query))
            .inner;

        if submitted && !self.query.trim().is_empty() {
            let q = self.query.clone();
            self.active_nav.clear();
            self.feed_label = format!("Результаты: {}", q);
            self.load_search(&q);
        }

        // ── сайдбар ─────────────────────────────────
        let nav = egui::SidePanel::left("sidebar")
            .exact_width(232.0)
            .frame(ui::theme::panel_frame(8.0))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .show(ui, |ui| ui::sidebar::show(ui, &self.active_nav))
                    .inner
            })
            .inner;

        if let Some(n) = nav {
            match n {
                ui::sidebar::Nav::Home => {
                    self.active_nav = "Главная".into();
                    self.feed_label = "Главная".into();
                    self.load_home();
                }
                ui::sidebar::Nav::Query(q) => {
                    self.active_nav = q.clone();
                    self.feed_label = q.clone();
                    self.load_search(&q);
                }
            }
        }

        // ── контент ─────────────────────────────────
        egui::CentralPanel::default()
            .frame(ui::theme::panel_frame(16.0))
            .show(ctx, |ui| {
                // Фон central panel с разделителем
                ui.painter().rect_filled(
                    ui.max_rect(),
                    egui::Rounding::same(0.0),
                    ui::theme::BG_PRIMARY,
                );
                
                if let Some(c) = ui::feed::chips(ui, &self.chip) {
                    self.chip = c.clone();
                    if c == "Все" {
                        self.feed_label = "Главная".into();
                        self.load_home();
                    } else {
                        self.feed_label = c.clone();
                        self.load_search(&c);
                    }
                }
                ui.add_space(8.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    // «Продолжить просмотр»
                    if !self.history.is_empty() {
                        ui.label(
                            egui::RichText::new("Продолжить просмотр")
                                .strong()
                                .size(18.0)
                                .color(ui::theme::TEXT),
                        );
                        ui.add_space(8.0);
                        let hist = self.history.clone();
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            ui.horizontal_top(|ui| {
                                for v in &hist {
                                    let t = self.textures.get(&v.key()).cloned();
                                    let act = ui::feed::card(ui, v, t.as_ref(), 260.0);
                                    self.handle_action(act, v);
                                    ui.add_space(16.0);
                                }
                            });
                        });
                        ui.add_space(24.0);
                    }

                    // Заголовок ленты + статусы
                    ui.label(
                        egui::RichText::new(&self.feed_label)
                            .strong()
                            .size(18.0)
                            .color(ui::theme::TEXT),
                    );
                    if self.loading {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label(egui::RichText::new("Загрузка...").color(ui::theme::GRAY));
                        });
                    }
                    if let Some(e) = &self.error {
                        ui.colored_label(egui::Color32::RED, format!("❌ {}", e));
                    }
                    ui.add_space(12.0);

                    // Адаптивная сетка карточек
                    let avail = ui.available_width();
                    let cols = ((avail + 16.0) / (300.0 + 16.0)).floor().max(1.0) as usize;
                    let card_w = (avail - (cols as f32 - 1.0) * 16.0) / cols as f32;

                    let feed = self.feed.clone();
                    for row in feed.chunks(cols) {
                        ui.horizontal_top(|ui| {
                            for v in row {
                                let t = self.textures.get(&v.key()).cloned();
                                let act = ui::feed::card(ui, v, t.as_ref(), card_w);
                                self.handle_action(act, v);
                                if card_w > 16.0 {
                                    ui.add_space(16.0);
                                }
                            }
                        });
                        ui.add_space(20.0);
                    }
                });
            });
    }
}
