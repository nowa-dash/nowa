//! Software decoding with OpenH264 (compiled into the binary).
use super::convert::{Yuv, show};
use crate::Display;
use openh264::decoder::Decoder;
use openh264::formats::YUVSource;
use std::sync::Arc;
use std::sync::mpsc::{SyncSender, sync_channel};

pub fn spawn(display: Arc<dyn Display>) -> SyncSender<Vec<u8>> {
    eprintln!("video: software decoder");
    let (tx, rx) = sync_channel::<Vec<u8>>(4);
    std::thread::spawn(move || {
        let mut dec = Decoder::new().expect("openh264 decoder");
        for h264 in rx {
            let Ok(Some(f)) = dec.decode(&h264) else {
                continue;
            };
            let (w, h) = f.dimensions();
            let (ys, us, vs) = f.strides();
            let yuv = Yuv::I420([(f.y(), ys), (f.u(), us), (f.v(), vs)]);
            show(&*display, w as u32, h as u32, yuv);
        }
    });
    tx
}
