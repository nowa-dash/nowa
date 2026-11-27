//! Video settings advertised to the phone.
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    /// VideoResolution enum: 1=480p 2=720p 3=1080p
    res: u64,
    /// UI density the phone renders at.
    pub dpi: u64,
}

impl Config {
    /// 480, 720 or 1080 lines; anything else means 480.
    pub fn new(lines: u32) -> Self {
        let (width, height, res, dpi) = match lines {
            1080 => (1920, 1080, 3, 240),
            720 => (1280, 720, 2, 160),
            _ => (800, 480, 1, 140), // what a Pi 3 decodes comfortably
        };
        Config {
            width,
            height,
            res,
            dpi,
        }
    }

    pub fn res(&self) -> u64 {
        self.res
    }
}

static CFG: OnceLock<Config> = OnceLock::new();

pub fn set(c: Config) {
    let _ = CFG.set(c);
}

pub fn cfg() -> &'static Config {
    CFG.get_or_init(|| Config::new(480))
}
