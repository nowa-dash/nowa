//! Reads frames from the phone, decrypts and reassembles them into messages.
use crate::Display;
use crate::channel::*;
use crate::conn::UsbIn;
use crate::dispatch::Services;
use std::io::Read;
use std::sync::Arc;

pub fn run(mut usb: UsbIn, display: Arc<dyn Display>) {
    let c = usb.conn.clone();
    let mut services = Services::new(c.clone(), display);
    let mut msgs = vec![Vec::new(); COUNT]; // reassembly per channel
    let mut frame = vec![0u8; 65536];
    let mut hdr = [0u8; 8];
    c.send(CONTROL, 1, &[0, 1, 0, 1]); // version 1.1
    while c.alive() && usb.read_exact(&mut hdr[..4]).is_ok() {
        let (ch, fl, len) = (
            hdr[0] as usize,
            hdr[1],
            u16::from_be_bytes([hdr[2], hdr[3]]) as usize,
        );
        if fl & 3 == F_FIRST && usb.read_exact(&mut hdr[4..]).is_err() {
            break; // total size, unused
        }
        if usb.read_exact(&mut frame[..len]).is_err() {
            break;
        }
        if ch >= COUNT {
            continue; // not a channel we advertised
        }
        let m = &mut msgs[ch];
        if fl & F_FIRST != 0 {
            m.clear();
        }
        if fl & F_ENC != 0 {
            if let Err(e) = c.decrypt(&frame[..len], m) {
                eprintln!("decrypt: {e}");
                break;
            }
        } else {
            m.extend_from_slice(&frame[..len]);
        }
        if fl & F_LAST != 0 {
            services.handle(ch as u8, m);
            m.clear();
        }
    }
    c.die();
    eprintln!("session ended");
}
