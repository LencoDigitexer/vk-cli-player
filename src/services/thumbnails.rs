// thumbnails.rs — загрузка и декодирование строго вне UI-потока
use egui::ColorImage;
use std::sync::mpsc;
use std::thread;

pub type Key = (i64, i64);
pub type Msg = (Key, Result<ColorImage, ()>);

pub struct ThumbnailLoader {
    tx: mpsc::Sender<Msg>,
    rx: mpsc::Receiver<Msg>,
}

impl ThumbnailLoader {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { tx, rx }
    }

    pub fn request(&self, key: Key, url: String) {
        let tx = self.tx.clone();
        thread::spawn(move || {
            let res = (|| -> Result<ColorImage, ()> {
                let bytes = reqwest::blocking::get(&url)
                    .map_err(|_| ())?
                    .bytes()
                    .map_err(|_| ())?;
                let img = image::load_from_memory(&bytes).map_err(|_| ())?;
                let img = img
                    .resize(320, 180, image::imageops::FilterType::Triangle)
                    .into_rgba8();
                let (w, h) = img.dimensions();
                Ok(ColorImage::from_rgba_unmultiplied(
                    [w as usize, h as usize],
                    img.as_raw(),
                ))
            })();
            let _ = tx.send((key, res));
        });
    }

    pub fn drain(&self) -> Vec<Msg> {
        let mut out = Vec::new();
        while let Ok(m) = self.rx.try_recv() {
            out.push(m);
        }
        out
    }
}
