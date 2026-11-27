//! Connects the music crate to the music page: playback controls and now-playing info.
use crate::AppWindow;
use music::{Player, Song, format_time};
use slint::{ComponentHandle, Weak};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn start(ui: &AppWindow) {
    let player = match Player::new(None) {
        Ok(p) => Arc::new(p),
        Err(e) => return eprintln!("music: no audio output: {e}"),
    };
    player.set_playing(false);

    let (p, w) = (player.clone(), ui.as_weak());
    ui.on_play_pause(move || {
        let playing = !p.is_playing();
        p.set_playing(playing);
        w.unwrap().set_is_playing(playing);
    });
    let w = ui.as_weak();
    ui.on_toggle_loop(move || {
        let ui = w.unwrap();
        ui.set_is_loop_enabled(!ui.get_is_loop_enabled());
    });
    let w = ui.as_weak();
    ui.on_toggle_shuffle(move || {
        let ui = w.unwrap();
        ui.set_is_shuffle_enabled(!ui.get_is_shuffle_enabled());
    });
    ui.on_next_track(|| {});
    ui.on_previous_track(|| {});
    ui.on_set_progress(|progress| println!("set progress to {}", progress.clamp(0.0, 1.0)));

    let w = ui.as_weak();
    thread::spawn(move || play_library(&player, &w));
}

/// Plays every song in the library once, keeping the now-playing info up to date.
fn play_library(player: &Player, ui: &Weak<AppWindow>) {
    for path in music::scan(music::library()) {
        while player.has_next_song() {
            thread::sleep(Duration::from_millis(100));
        }
        let Ok(song) = Song::from_file(&path, None) else {
            continue;
        };
        if player.play_song_next(&song, None).is_err() {
            continue;
        }
        while player.has_next_song() {
            thread::sleep(Duration::from_millis(100));
        }
        let Some(track) = music::track(&path) else {
            continue;
        };
        let duration = track.duration.as_secs();
        let _ = ui.upgrade_in_event_loop(move |ui| {
            ui.set_track_title(track.title.into());
            ui.set_artist_name(track.artist.into());
            ui.set_album_name(track.album.into());
            ui.set_total_time(format_time(duration).into());
        });
        while player.has_current_song() {
            if let Some((elapsed, _)) = player.get_playback_position() {
                let secs = elapsed.as_secs();
                let progress = if duration > 0 {
                    secs as f32 / duration as f32
                } else {
                    0.0
                };
                let _ = ui.upgrade_in_event_loop(move |ui| {
                    ui.set_current_time(format_time(secs).into());
                    ui.set_progress(progress);
                });
            }
            thread::sleep(Duration::from_secs(1));
        }
    }
}
