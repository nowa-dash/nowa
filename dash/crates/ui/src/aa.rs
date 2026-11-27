//! Connects the android-auto crate to the Android Auto page: frames in, touches out.
use crate::AppWindow;
use android_auto::{Config, Display, Touch};
use slint::{ComponentHandle, Image, Rgb8Pixel, SharedPixelBuffer, Weak};
use std::sync::{Arc, Mutex};

type Rgb = SharedPixelBuffer<Rgb8Pixel>;

pub fn start(ui: &AppWindow) {
    // AA_RES=720 or 1080 for bigger screens; 480 is what a Pi 3 decodes comfortably
    let lines = std::env::var("AA_RES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(480);
    let cfg = Config::new(lines);
    ui.set_android_auto_width(cfg.width as i32);
    ui.set_android_auto_height(cfg.height as i32);
    ui.on_handle_android_auto_touch(|x, y, action| {
        let action = match action {
            0 => Touch::Down,
            1 => Touch::Up,
            _ => Touch::Move,
        };
        android_auto::touch(0, action, x as f32, y as f32);
    });
    android_auto::start(
        cfg,
        Arc::new(Screen {
            ui: ui.as_weak(),
            frames: Arc::default(),
        }),
    );
}

struct Screen {
    ui: Weak<AppWindow>,
    frames: Arc<Frames>,
}

#[derive(Default)]
struct Frames {
    /// Newest frame not yet shown; Some means an update is already queued on the UI thread.
    pending: Mutex<Option<Rgb>>,
    /// The previously shown frame's buffer, reused instead of allocating 1MB+ per frame.
    spare: Mutex<Option<Rgb>>,
}

impl Display for Screen {
    fn connected(&self, _on: bool) {
        self.frames.pending.lock().unwrap().take();
        let _ = self
            .ui
            .upgrade_in_event_loop(|ui| ui.set_android_auto_frame(Image::default()));
    }

    fn frame(&self, width: u32, height: u32, draw: &mut dyn FnMut(&mut [u8])) {
        let mut buf = match self.frames.spare.lock().unwrap().take() {
            Some(b) if b.width() == width && b.height() == height => b,
            _ => Rgb::new(width, height),
        };
        draw(buf.make_mut_bytes());
        if self.frames.pending.lock().unwrap().replace(buf).is_some() {
            return; // an update is already queued; it picks up this newer frame
        }
        let frames = self.frames.clone();
        let _ = self.ui.upgrade_in_event_loop(move |ui| {
            let Some(f) = frames.pending.lock().unwrap().take() else {
                return;
            };
            let old = ui.get_android_auto_frame();
            ui.set_android_auto_frame(Image::from_rgb8(f));
            *frames.spare.lock().unwrap() = old.to_rgb8();
        });
    }
}
