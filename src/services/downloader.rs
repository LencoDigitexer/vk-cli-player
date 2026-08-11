// downloader.rs
use std::path::PathBuf;
use std::process::Command;
use std::thread;

pub fn start(url: &str) {
    let dir = dirs::download_dir().unwrap_or_else(|| PathBuf::from("."));
    let out = dir.join("%(title)s.%(ext)s");
    let url = url.to_string();

    thread::spawn(move || {
        let status = Command::new("yt-dlp")
            .args([
                "-o",
                out.to_str().unwrap(),
                "-f",
                "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best",
                "--merge-output-format",
                "mp4",
                &url,
            ])
            .status();

        match status {
            Ok(s) if s.success() => println!("✅ Сохранено в {}", dir.display()),
            Ok(_) => eprintln!("❌ Ошибка скачивания"),
            Err(e) => eprintln!("❌ yt-dlp: {}", e),
        }
    });
}
