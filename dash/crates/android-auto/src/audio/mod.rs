//! Audio through PipeWire/PulseAudio (native protocol, pure Rust): speakers and microphone.
use crate::channel::*;
use pulseaudio::Client;
use pulseaudio::protocol::{ChannelMap, SampleFormat, SampleSpec};
use std::sync::OnceLock;

mod mic;
mod speaker;

pub use mic::Mic;
pub use speaker::Speaker;

/// (channel, AudioType, rate, channels): advertised in service discovery and used for the streams.
pub const FORMATS: [(u8, u64, u32, u8); 4] = [
    (MEDIA, 3, 48000, 2),
    (SPEECH, 1, 16000, 1),
    (SYSTEM, 2, 16000, 1),
    (MIC, 0, 16000, 1),
];

pub fn client() -> Option<&'static Client> {
    static C: OnceLock<Option<Client>> = OnceLock::new();
    C.get_or_init(|| {
        Client::from_env(c"headunit")
            .map_err(|e| eprintln!("audio: {e}"))
            .ok()
    })
    .as_ref()
}

pub fn spec(ch: u8) -> (SampleSpec, ChannelMap, usize) {
    let (_, _, rate, chans) = FORMATS.into_iter().find(|f| f.0 == ch).unwrap();
    let map = if chans == 2 {
        ChannelMap::stereo()
    } else {
        ChannelMap::mono()
    };
    let spec = SampleSpec {
        format: SampleFormat::S16Le,
        channels: chans,
        sample_rate: rate,
    };
    (spec, map, rate as usize * chans as usize * 2)
}
