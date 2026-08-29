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
use std::collections::HashMap;

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
                let builder_res = zbus::blocking::fdo::PropertiesProxy::builder(conn)
                    .destination(name.clone());
                let builder = match builder_res {
                    Ok(b) => match b.path("/org/mpris/MediaPlayer2") {
                        Ok(p) => p,
                        Err(_) => continue,
                    },
                    Err(_) => continue,
                };

                if let Ok(proxy) = builder.build() {
                    let status = proxy
                        .get::<String>("org.mpris.MediaPlayer2.Player", "PlaybackStatus")
                        .unwrap_or_default();

                    let is_playing = status == "Playing";

                    let mut title = String::new();
                    let mut artist = String::new();
                    let mut album = String::new();
                    let mut art_url = None;

                    if let Ok(map) = proxy.get::<HashMap<String, zbus::zvariant::OwnedValue>>(
                        "org.mpris.MediaPlayer2.Player",
                        "Metadata",
                    ) {
                        if let Some(t) = map.get("xesam:title") {
                            if let Ok(s) = String::try_from(t.clone()) {
                                title = s;
                            }
                        }
                        if let Some(a) = map.get("xesam:artist") {
                            if let Ok(arr) = Vec::<String>::try_from(a.clone()) {
                                artist = arr.join(", ");
                            } else if let Ok(s) = String::try_from(a.clone()) {
                                artist = s;
                            }
                        }
                        if let Some(al) = map.get("xesam:album") {
                            if let Ok(s) = String::try_from(al.clone()) {
                                album = s;
                            }
                        }
                        if let Some(url) = map.get("mpris:artUrl") {
                            if let Ok(s) = String::try_from(url.clone()) {
                                art_url = Some(s);
                            }
                        }
                    }

                    let app_name = name_str
                        .trim_start_matches("org.mpris.MediaPlayer2.")
                        .to_string();

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
