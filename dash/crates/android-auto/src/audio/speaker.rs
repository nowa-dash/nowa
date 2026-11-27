//! A playback stream for one AA audio channel.
use super::{client, spec};
use futures::executor::block_on;
use pulseaudio::protocol::PlaybackStreamParams;
use pulseaudio::{AsPlaybackSource, PlaybackStream};
use std::collections::VecDeque;
use std::io::Read;
use std::sync::{Arc, Mutex};

/// A playback stream fed from the USB thread; plays silence when the phone sends nothing.
pub struct Speaker {
    queue: Arc<Mutex<VecDeque<u8>>>,
    _stream: PlaybackStream, // deleted on drop by the pulseaudio crate
    max: usize,
}

impl Speaker {
    pub fn new(ch: u8) -> Option<Self> {
        let (sample_spec, channel_map, bytes_per_sec) = spec(ch);
        let queue = Arc::new(Mutex::new(VecDeque::new()));
        let q = queue.clone();
        let source = move |out: &mut [u8]| {
            let mut q = q.lock().unwrap();
            // two reads: VecDeque's Read only takes the front slice, the data may wrap around
            let n = q.read(out).unwrap_or(0);
            let n = n + q.read(&mut out[n..]).unwrap_or(0);
            out[n..].fill(0);
            out.len()
        };
        let params = PlaybackStreamParams {
            sample_spec,
            channel_map,
            ..Default::default()
        };
        let stream =
            block_on(client()?.create_playback_stream(params, source.as_playback_source()))
                .map_err(|e| eprintln!("audio out: {e}"))
                .ok()?;
        Some(Speaker {
            queue,
            _stream: stream,
            max: bytes_per_sec / 2,
        })
    }

    pub fn push(&self, pcm: &[u8]) {
        let mut q = self.queue.lock().unwrap();
        q.extend(pcm);
        let extra = q.len().saturating_sub(self.max); // never fall more than 0.5s behind
        q.drain(..extra);
    }
}
