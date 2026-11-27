//! Channel ids we advertise to the phone, and frame header flags.

pub const CONTROL: u8 = 0;
pub const INPUT: u8 = 1;
pub const SENSOR: u8 = 2;
pub const VIDEO: u8 = 3;
pub const MEDIA: u8 = 4;
pub const SPEECH: u8 = 5;
pub const SYSTEM: u8 = 6;
pub const MIC: u8 = 7;
pub const COUNT: usize = 8;

pub const F_FIRST: u8 = 1;
pub const F_LAST: u8 = 2;
pub const F_CTRL: u8 = 4;
pub const F_ENC: u8 = 8;
