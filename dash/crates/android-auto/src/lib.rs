//! Wired Android Auto head unit: USB, TLS, the protocol, H.264 decoding and audio.
//! The UI is not part of this crate: it implements [`Display`] and forwards touches with [`touch`].
//! Protocol follows aasdk/openauto.

mod audio;
mod channel;
mod config;
mod conn;
mod control;
mod discovery;
mod dispatch;
mod pb;
mod session;
mod tls;
mod touch;
mod usb;
mod video;

pub use config::Config;
pub use touch::{Touch, touch};

use std::sync::Arc;

/// Where video goes. Called from background threads.
pub trait Display: Send + Sync + 'static {
    /// A phone connected or disconnected.
    fn connected(&self, on: bool);
    /// One decoded frame: `draw` fills a `width * height * 3` byte RGB buffer you provide.
    fn frame(&self, width: u32, height: u32, draw: &mut dyn FnMut(&mut [u8]));
}

/// Starts looking for phones on a background thread; runs a session whenever one is plugged in.
pub fn start(config: Config, display: Arc<dyn Display>) {
    config::set(config);
    std::thread::spawn(move || usb::run(display));
}
