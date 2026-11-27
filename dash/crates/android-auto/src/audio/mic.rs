//! The microphone, used for voice commands.
use super::{client, spec};
use crate::channel::MIC;
use crate::conn::Conn;
use futures::executor::block_on;
use pulseaudio::RecordStream;
use pulseaudio::protocol::RecordStreamParams;
use std::sync::Arc;
use std::time::Instant;

const MIC_CHUNK: usize = 2048; // 64ms of 16kHz mono s16

/// Microphone recording, sent to the phone in fixed chunks with a timestamp.
pub struct Mic(RecordStream);

impl Mic {
    pub fn new(c: Arc<Conn>) -> Option<Self> {
        let (sample_spec, channel_map, _) = spec(MIC);
        let start = Instant::now();
        let mut pcm = Vec::new();
        let sink = move |data: &[u8]| {
            pcm.extend_from_slice(data);
            while pcm.len() >= MIC_CHUNK {
                let mut m = Vec::with_capacity(8 + MIC_CHUNK);
                m.extend_from_slice(&(start.elapsed().as_micros() as u64).to_be_bytes());
                m.extend(pcm.drain(..MIC_CHUNK));
                c.send(MIC, 0x0000, &m);
            }
        };
        let params = RecordStreamParams {
            sample_spec,
            channel_map,
            ..Default::default()
        };
        let stream = block_on(client()?.create_record_stream(params, sink))
            .map_err(|e| eprintln!("mic: {e}"))
            .ok()?;
        Some(Mic(stream))
    }

    pub fn set(&self, on: bool) {
        let _ = if on {
            block_on(self.0.uncork())
        } else {
            block_on(self.0.cork())
        };
    }
}
