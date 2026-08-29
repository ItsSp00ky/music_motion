use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LinuxMediaInfo {
    pub is_available: bool,
    pub is_playing: bool,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub source_app: String,
    pub thumbnail_url: Option<String>,
}

#[cfg(target_os = "linux")]
pub struct LinuxMprisMonitor;

#[cfg(target_os = "linux")]
impl LinuxMprisMonitor {
    pub fn new() -> Self {
        Self
    }

    pub fn get_current_media_info(&mut self) -> Option<LinuxMediaInfo> {
        let finder = mpris::PlayerFinder::new().ok()?;
        let players = finder.find_all().ok()?;

        let mut playing_info = None;
        let mut fallback_info = None;

        for player in players {
            let status = player.get_playback_status().unwrap_or(mpris::PlaybackStatus::Stopped);
            let is_playing = status == mpris::PlaybackStatus::Playing;
            let identity = player.identity().to_string();

            if let Ok(metadata) = player.get_metadata() {
                let title = metadata.title().unwrap_or_default().to_string();
                let artists = metadata.artists().unwrap_or_default();
                let artist = artists.join(", ");
                let album = metadata.album_name().unwrap_or_default().to_string();
                let art_url = metadata.art_url().map(|s| s.to_string());

                let info = LinuxMediaInfo {
                    is_available: !title.is_empty(),
                    is_playing,
                    title,
                    artist,
                    album,
                    source_app: identity,
                    thumbnail_url: art_url,
                };

                if is_playing {
                    playing_info = Some(info);
                    break;
                } else if fallback_info.is_none() && info.is_available {
                    fallback_info = Some(info);
                }
            }
        }

        playing_info.or(fallback_info)
    }
}

#[cfg(not(target_os = "linux"))]
pub struct LinuxMprisMonitor;

#[cfg(not(target_os = "linux"))]
impl LinuxMprisMonitor {
    pub fn new() -> Self {
        Self
    }
    pub fn get_current_media_info(&mut self) -> Option<LinuxMediaInfo> {
        None
    }
}
