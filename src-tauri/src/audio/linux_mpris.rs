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
pub struct LinuxMprisMonitor {
    conn: Option<zbus::blocking::Connection>,
}

#[cfg(target_os = "linux")]
impl LinuxMprisMonitor {
    pub fn new() -> Self {
        let conn = zbus::blocking::Connection::session().ok();
        Self { conn }
    }

    pub fn get_current_media_info(&mut self) -> Option<LinuxMediaInfo> {
        let conn = self.conn.as_ref()?;
        let dbus_proxy = zbus::blocking::fdo::DBusProxy::new(conn).ok()?;
        let names = dbus_proxy.list_names().ok()?;

        let mut playing_info = None;
        let mut fallback_info = None;

        for name in names {
            let name_str = name.as_str();
            if name_str.starts_with("org.mpris.MediaPlayer2.") {
                if let Ok(proxy) = zbus::blocking::fdo::PropertiesProxy::builder(conn)
                    .destination(name.clone())
                    .path("/org/mpris/MediaPlayer2")
                {
                    if let Ok(proxy) = proxy.build() {
                        let status: String = proxy
                            .get("org.mpris.MediaPlayer2.Player", "PlaybackStatus")
                            .map(|v: zbus::zvariant::Value| v.downcast_into().unwrap_or_default())
                            .unwrap_or_default();

                        let is_playing = status == "Playing";

                        let mut title = String::new();
                        let mut artist = String::new();
                        let mut album = String::new();
                        let mut art_url = None;

                        if let Ok(metadata_val) = proxy.get::<zbus::zvariant::Value>("org.mpris.MediaPlayer2.Player", "Metadata") {
                            if let Ok(map) = <std::collections::HashMap<String, zbus::zvariant::Value>>::try_from(metadata_val) {
                                if let Some(t) = map.get("xesam:title") {
                                    title = t.downcast_ref::<str>().unwrap_or_default().to_string();
                                }
                                if let Some(a) = map.get("xesam:artist") {
                                    if let Some(arr) = a.downcast_ref::<zbus::zvariant::Array>() {
                                        let artists: Vec<String> = arr.iter().filter_map(|v| v.downcast_ref::<str>().map(|s| s.to_string())).collect();
                                        artist = artists.join(", ");
                                    } else if let Some(s) = a.downcast_ref::<str>() {
                                        artist = s.to_string();
                                    }
                                }
                                if let Some(al) = map.get("xesam:album") {
                                    album = al.downcast_ref::<str>().unwrap_or_default().to_string();
                                }
                                if let Some(url) = map.get("mpris:artUrl") {
                                    art_url = url.downcast_ref::<str>().map(|s| s.to_string());
                                }
                            }
                        }

                        let app_name = name_str.trim_start_matches("org.mpris.MediaPlayer2.").to_string();

                        let info = LinuxMediaInfo {
                            is_available: !title.is_empty(),
                            is_playing,
                            title,
                            artist,
                            album,
                            source_app: app_name,
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
