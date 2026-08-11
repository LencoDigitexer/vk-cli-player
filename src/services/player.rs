// player.rs
use std::io;
use std::process::{Child, Command};

pub fn play(url: &str, title: &str) -> io::Result<Child> {
    Command::new("mpv")
        .arg(url)
        .arg("--force-window=immediate")
        .arg(format!("--title=VK: {}", title))
        .spawn()
}
