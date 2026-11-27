//! H.264 decoding on its own thread: the V4L2 hardware decoder when the board has one,
//! else OpenH264 in software. Decoded frames go to the [`Display`] as RGB.
mod convert;
mod hw;
mod sw;

use crate::Display;
use std::sync::Arc;
use std::sync::mpsc::SyncSender;

/// Sends access units to the decoder thread.
pub struct Decoder(SyncSender<Vec<u8>>);

impl Decoder {
    /// AA_DECODER=sw forces the software decoder.
    pub fn new(display: Arc<dyn Display>) -> Self {
        let try_hw = std::env::var("AA_DECODER").as_deref() != Ok("sw");
        let hw = try_hw.then(|| hw::spawn(display.clone())).flatten();
        Decoder(hw.unwrap_or_else(|| sw::spawn(display)))
    }

    /// Blocks when the decoder is behind, which slows the phone down instead of dropping data.
    pub fn decode(&self, h264: &[u8]) {
        let _ = self.0.send(h264.to_vec());
    }
}
