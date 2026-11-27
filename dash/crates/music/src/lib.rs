//! Local music: finds songs on disk, reads their tags, plays them.
use lofty::prelude::*;
use lofty::probe::Probe;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub use playback_rs::{Player, Song};

/// The music folder: `MUSIC_DIR` if set, else where the music partition is mounted on the device.
pub fn library() -> PathBuf {
    std::env::var_os("MUSIC_DIR").map_or_else(|| "/storage/music".into(), PathBuf::from)
}

pub struct Track {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: Duration,
}

/// Songs laid out as `root/artist/album/song`. Unreadable folders are skipped.
pub fn scan(root: impl AsRef<Path>) -> Vec<PathBuf> {
    let dirs = |p: &Path| -> Vec<PathBuf> {
        let mut v: Vec<_> = std::fs::read_dir(p)
            .into_iter()
            .flatten()
            .flatten()
            .map(|e| e.path())
            .collect();
        v.sort();
        v
    };
    let mut songs = Vec::new();
    for artist in dirs(root.as_ref()) {
        for album in dirs(&artist) {
            songs.extend(dirs(&album));
        }
    }
    songs
}

/// Tags and length of a song; missing tags read as "None".
pub fn track(path: &Path) -> Option<Track> {
    let file = Probe::open(path).ok()?.read().ok()?;
    let tag = file.primary_tag().or_else(|| file.first_tag());
    let get = |f: fn(&lofty::tag::Tag) -> Option<std::borrow::Cow<'_, str>>| {
        tag.and_then(f)
            .map_or("None".to_string(), |s| s.into_owned())
    };
    Some(Track {
        title: get(|t| t.title()),
        artist: get(|t| t.artist()),
        album: get(|t| t.album()),
        duration: file.properties().duration(),
    })
}

pub fn format_time(secs: u64) -> String {
    format!("{}:{:02}", secs / 60, secs % 60)
}
