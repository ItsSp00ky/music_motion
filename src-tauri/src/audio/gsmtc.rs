use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession,
    GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
};
use windows::Storage::Streams::{DataReader, InputStreamOptions};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MediaSessionInfo {
    pub is_available: bool,
    pub is_playing: bool,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub source_app: String,
    pub thumbnail_base64: Option<String>,
}

pub struct GsmtcMonitor;

impl GsmtcMonitor {
    pub fn new() -> Self {
        Self
    }

    pub fn get_current_media_info(&self) -> Option<MediaSessionInfo> {
        let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().ok()?.get().ok()?;
        let session: GlobalSystemMediaTransportControlsSession = manager.GetCurrentSession().ok()?;

        let source_app = session
            .SourceAppUserModelId()
            .map(|s| s.to_string())
            .unwrap_or_else(|_| "Music Player".to_string());

        let playback_info = session.GetPlaybackInfo().ok()?;
        let status = playback_info.PlaybackStatus().ok()?;
        let is_playing = status == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing;

        let media_props = session.TryGetMediaPropertiesAsync().ok()?.get().ok()?;
        let title = media_props.Title().map(|s| s.to_string()).unwrap_or_default();
        let artist = media_props.Artist().map(|s| s.to_string()).unwrap_or_default();
        let album = media_props.AlbumTitle().map(|s| s.to_string()).unwrap_or_default();

        let mut thumbnail_base64 = None;
        if let Ok(thumbnail_ref) = media_props.Thumbnail() {
            if let Ok(stream) = thumbnail_ref.OpenReadAsync().and_then(|op| op.get()) {
                if let Ok(size) = stream.Size() {
                    if size > 0 && size < 5 * 1024 * 1024 {
                        let size_usize = size as usize;
                        let mut buffer = vec![0u8; size_usize];
                        if let Ok(reader) = DataReader::CreateDataReader(&stream) {
                            reader.SetInputStreamOptions(InputStreamOptions::None).ok();
                            if reader.LoadAsync(size as u32).and_then(|op| op.get()).is_ok() {
                                if reader.ReadBytes(&mut buffer).is_ok() {
                                    let encoded = STANDARD.encode(&buffer);
                                    thumbnail_base64 = Some(format!("data:image/jpeg;base64,{}", encoded));
                                }
                            }
                        }
                    }
                }
            }
        }

        Some(MediaSessionInfo {
            is_available: !title.is_empty() || !artist.is_empty(),
            is_playing,
            title,
            artist,
            album,
            source_app,
            thumbnail_base64,
        })
    }
}
