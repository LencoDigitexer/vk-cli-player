<div align="center">

# vk-video-native

Нативный десктопный клиент VK Video.

**Rust · egui · mpv · yt-dlp** — без WebView, Electron и браузера.

[![Rust](https://img.shields.io/badge/rust-stable-orange?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![GUI](https://img.shields.io/badge/GUI-egui-6441a5)](https://github.com/emilk/egui)
[![Player](https://img.shields.io/badge/player-mpv-691F70)](https://mpv.io)
[![Platform](https://img.shields.io/badge/platform-Linux%20%C2%B7%20Windows%20%C2%B7%20macOS-lightgrey)]()
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

![screenshot](docs/screenshot.png)

</div>

---

## Возможности

Интерфейс повторяет vkvideo.ru и полностью нативный:

- шапка с поиском, боковое меню (Стримы, Главная, Тренды, Клипы, История…),
  чипсы категорий, адаптивная сетка карточек
- карточка видео: превью 16:9 со скелетон-загрузкой, бейдж длительности,
  просмотры и дата публикации («640 тыс просмотров · 5 месяцев назад»)
- «Продолжить просмотр» — локальная история воспроизведения
- поиск и категории через `video.search`; «Главная» — дайджест популярных
  категорий (`video.getCatalog` удалён из VK API)
- защита от гонок запросов: запоздалый ответ старого запроса отбрасывается,
  лента всегда соответствует последнему действию
- воспроизведение в **mpv** по `direct_url` из ответа API
- скачивание в «Загрузки»: `bestvideo+bestaudio → mp4` (yt-dlp + ffmpeg)
- UI-поток не занимается сетью и декодированием: превью качаются и декодируются
  в фоновых потоках, данные приходят через `mpsc`-каналы

## Структура проекта

Интерфейс отделён от логики: `ui/*` только рисует и возвращает действия,
`api/*` и `services/*` не знают о UI, `app.rs` — единственный слой композиции.

```
src/
├── main.rs               # точка входа: eframe + применение темы
├── app.rs                # композиция: каналы, поколения запросов, роутинг действий
├── models.rs             # VideoItem, превью, форматирование (длительность/просмотры/дата)
├── api/
│   └── vk.rs             # VK API: video.search, дайджест «Главной»
├── services/
│   ├── player.rs         # mpv (subprocess)
│   ├── downloader.rs     # yt-dlp (subprocess, склейка видео+аудио)
│   ├── thumbnails.rs     # фоновые загрузка и JPEG-декод превью
│   └── history.rs        # «Продолжить просмотр» (.vk_history.json)
└── ui/
    ├── theme.rs          # палитра VK, панели без теней и рамок
    ├── topbar.rs         # лого + поиск
    ├── sidebar.rs        # боковое меню как на vkvideo.ru
    └── feed.rs           # чипсы категорий, карточка видео, скелетоны
```

## Требования

| Компонент | Назначение | Установка (Linux) |
|---|---|---|
| `mpv` | воспроизведение | `sudo apt install mpv` |
| `yt-dlp` | извлечение стримов | `sudo apt install yt-dlp` |
| `ffmpeg` | склейка дорожек при скачивании | `sudo apt install ffmpeg` |
| Rust toolchain | сборка | `rustup` |

## Сборка и запуск

```bash
# Linux
sudo apt install mpv yt-dlp ffmpeg libssl-dev pkg-config
cargo build --release
./target/release/vk-video-native
```

```bash
# macOS
brew install mpv yt-dlp ffmpeg
cargo build --release
```

```powershell
# Windows: mpv, yt-dlp и ffmpeg должны быть в PATH
cargo build --release
```

## Авторизация

Приложение работает с пользовательским токеном с правами `video`.
Токен хранится в файле `.vk_token` в рабочей директории и передаётся только в `api.vk.com`.

```bash
echo "<ВАШ_ТОКЕН>" > .vk_token
chmod 600 .vk_token
```

Получение токена: [vkhost.github.io](https://vkhost.github.io) — выберите приложение,
отметьте `video` + `offline`, скопируйте `access_token` из redirect-URL.

> Сервисный ключ приложения не подходит: `video.search` недоступен с ним (ошибка API 28).

## Использование

| Действие | Результат |
|---|---|
| `Enter` в поиске | поиск видео |
| пункт сайдбара / чипс категории | лента по разделу |
| клик по карточке | воспроизведение в mpv |
| «Скачать» | загрузка mp4 в `~/Downloads` |
| закрытие окна mpv | возврат к ленте |

## Архитектура

```
              ┌────────────────────────────────────────────┐
              │             app.rs (композиция)            │
              │  feed_gen: применяется только последний    │
              │  запрос; действия UI → сервисы             │
              └──────▲──────────────▲──────────────▲───────┘
               mpsc  │        mpsc  │              │ действия UI
        ┌────────────┴───┐  ┌───────┴────────┐  ┌──┴───────────────┐
        │ api/vk.rs      │  │ thumbnails.rs  │  │ ui/* (egui)      │
        │ reqwest blocking│ │ загрузка+JPEG  │  │ topbar, sidebar, │
        │ video.search   │  │ decode в фоне  │  │ feed, скелетоны  │
        └────────────────┘  └────────────────┘  └──────────────────┘
        ────────────────┐  ┌────────────────┐  ┌──────────────────┐
        │ player.rs      │  │ downloader.rs  │  │ history.rs       │
        │ mpv subprocess │  │ yt-dlp + ffmpeg│  │ .vk_history.json │
        └────────────────  └────────────────┘  └──────────────────┘
```

| Подсистема | Реализация |
|---|---|
| GUI | `egui` / `eframe`, immediate mode, GPU-рендер |
| VK API | `reqwest::blocking` в фоновых потоках |
| Превью | фоновые потоки + `image` (JPEG), кэш в `HashMap`, скелетоны |
| Воспроизведение | subprocess `mpv`, ссылка `direct_url` из API |
| Скачивание | subprocess `yt-dlp`, `bestvideo+bestaudio → mp4` |
| История | локальный JSON, без сервера |

## Roadmap

- [ ] тёмная тема (переключение)
- [ ] кэш превью на диске
- [ ] реальные разделы: плейлисты, «Мне понравилось», подписки
- [ ] стримы (LIVE-бейджи и HLS)
- [ ] встроенный плеер (рендер кадров mpv в текстуру egui)
- [ ] CI-релизы для Linux / Windows / macOS

## Дисклеймер

Не является официальным продуктом VK. Использует публичное VK API от имени
пользователя. Уважайте [условия использования VK API](https://dev.vk.com/ru/rules)
и права правообладателей контента.

## Лицензия

[MIT](LICENSE)
