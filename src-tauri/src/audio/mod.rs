pub mod gsmtc;
pub mod wasapi;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use gsmtc::{GsmtcMonitor, MediaSessionInfo};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use wasapi::{ProcessAudioInfo, WasapiMonitor};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AudioState {
    pub has_audio: bool,
    pub overall_peak: f32,
    pub is_playing: bool,
    pub track_title: String,
    pub artist: String,
    pub album: String,
    pub thumbnail: Option<String>,
    pub source_app: String,
    pub active_apps: Vec<ProcessAudioInfo>,
}

pub struct AudioEngine {
    running: Arc<AtomicBool>,
}

impl AudioEngine {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn start_polling(&self, app_handle: AppHandle) {
        let running = self.running.clone();

        std::thread::spawn(move || {
            let wasapi = WasapiMonitor::new();
            let gsmtc = GsmtcMonitor::new();

            let mut cached_media: Option<MediaSessionInfo> = None;
            let mut media_poll_tick = 0u32;

            while running.load(Ordering::Relaxed) {
                // Query WASAPI peak meter at ~30Hz (fast)
                let (peak, active_apps) = wasapi.get_active_sessions();

                // Poll GSMTC less aggressively (~5Hz / every 6 ticks) to keep CPU at ~0%
                media_poll_tick = (media_poll_tick + 1) % 6;
                if media_poll_tick == 0 || cached_media.is_none() {
                    cached_media = gsmtc.get_current_media_info();
                }

                let mut state = AudioState {
                    overall_peak: peak,
                    has_audio: peak > 0.005,
                    active_apps,
                    ..Default::default()
                };

                if let Some(media) = &cached_media {
                    if media.is_available {
                        state.track_title = media.title.clone();
                        state.artist = media.artist.clone();
                        state.album = media.album.clone();
                        state.thumbnail = media.thumbnail_base64.clone();
                        state.source_app = media.source_app.clone();
                        state.is_playing = media.is_playing;
                        if media.is_playing {
                            state.has_audio = true;
                        }
                    }
                }

                // If no media title is found, use top active audio process as source_app
                if state.source_app.is_empty() && !state.active_apps.is_empty() {
                    let top_app = state.active_apps.iter().max_by(|a, b| {
                        a.peak.partial_cmp(&b.peak).unwrap_or(std::cmp::Ordering::Equal)
                    });
                    if let Some(top) = top_app {
                        state.source_app = top.name.clone();
                        state.track_title = top.name.clone();
                        state.artist = format!("PID {}", top.pid);
                    }
                }

                // Emit audio-update event to overlay frontend
                let _ = app_handle.emit("audio-update", &state);

                std::thread::sleep(Duration::from_millis(33)); // ~30 FPS
            }
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}
