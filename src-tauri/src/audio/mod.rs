pub mod gsmtc;
pub mod linux_mpris;
pub mod wasapi;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
use wasapi::ProcessAudioInfo;
use crate::window::WindowHelper;

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
            #[cfg(target_os = "windows")]
            let wasapi = wasapi::WasapiMonitor::new();
            #[cfg(target_os = "windows")]
            let gsmtc = gsmtc::GsmtcMonitor::new();
            #[cfg(target_os = "windows")]
            let mut cached_media: Option<gsmtc::MediaSessionInfo> = None;

            #[cfg(target_os = "linux")]
            let mut linux_mpris = linux_mpris::LinuxMprisMonitor::new();
            #[cfg(target_os = "linux")]
            let mut cached_linux_media: Option<linux_mpris::LinuxMediaInfo> = None;
            #[cfg(target_os = "linux")]
            let mut linux_sim_angle = 0.0f32;

            let mut media_poll_tick = 0u32;
            let mut topmost_tick = 0u32;

            while running.load(Ordering::Relaxed) {
                topmost_tick = (topmost_tick + 1) % 60; // Every ~2 seconds (60 * 33ms)
                if topmost_tick == 0 {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        WindowHelper::reassert_topmost(&window);
                    }
                }

                #[cfg(target_os = "windows")]
                {
                    let (peak, active_apps) = wasapi.get_active_sessions();

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

                    let _ = app_handle.emit("audio-update", &state);
                }

                #[cfg(target_os = "linux")]
                {
                    media_poll_tick = (media_poll_tick + 1) % 6;
                    if media_poll_tick == 0 || cached_linux_media.is_none() {
                        cached_linux_media = linux_mpris.get_current_media_info();
                    }

                    let mut state = AudioState::default();

                    if let Some(media) = &cached_linux_media {
                        if media.is_available {
                            state.track_title = media.title.clone();
                            state.artist = media.artist.clone();
                            state.album = media.album.clone();
                            state.source_app = media.source_app.clone();
                            state.thumbnail = media.thumbnail_url.clone();
                            state.is_playing = media.is_playing;
                            state.has_audio = media.is_playing;

                            if media.is_playing {
                                linux_sim_angle += 0.12;
                                let simulated_peak = (linux_sim_angle.sin() * 0.4 + 0.6)
                                    * ((linux_sim_angle * 2.3).sin() * 0.3 + 0.7);
                                state.overall_peak = simulated_peak.clamp(0.15, 0.95);
                            }

                            state.active_apps.push(ProcessAudioInfo {
                                name: media.source_app.clone(),
                                pid: 0,
                                peak: state.overall_peak,
                            });
                        }
                    }

                    let _ = app_handle.emit("audio-update", &state);
                }

                #[cfg(not(any(target_os = "windows", target_os = "linux")))]
                {
                    let state = AudioState::default();
                    let _ = app_handle.emit("audio-update", &state);
                }

                std::thread::sleep(Duration::from_millis(33)); // ~30 FPS
            }
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}
