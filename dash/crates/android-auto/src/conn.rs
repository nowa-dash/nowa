//! One USB connection to a phone: framing and encryption of outgoing messages, plus TLS state.
use crate::channel::*;
use crate::pb::Pb;
use crate::tls::Tls;
use nusb::io::{EndpointRead, EndpointWrite};
use nusb::transfer::Bulk;
use std::io::{self, Read, Write};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::sync::{Arc, Mutex};

/// The phone currently connected, for senders outside the session thread (touch input).
pub static CURRENT: Mutex<Option<Arc<Conn>>> = Mutex::new(None);

pub struct Conn {
    alive: AtomicBool,
    /// TLS state lives with the writer: both directions share it, and it keeps records in order.
    out: Mutex<Out>,
}

struct Out {
    usb: EndpointWrite<Bulk>,
    tls: Option<Tls>,
    plain: Vec<u8>,
    frame: Vec<u8>,
}

fn no_tls() -> io::Error {
    io::Error::new(
        io::ErrorKind::NotConnected,
        "encrypted data before TLS handshake",
    )
}

impl Conn {
    pub fn new(usb: EndpointWrite<Bulk>) -> Arc<Self> {
        let out = Out {
            usb,
            tls: None,
            plain: Vec::new(),
            frame: Vec::new(),
        };
        Arc::new(Conn {
            alive: AtomicBool::new(true),
            out: Mutex::new(out),
        })
    }

    pub fn alive(&self) -> bool {
        self.alive.load(Relaxed)
    }

    pub fn die(&self) {
        self.alive.store(false, Relaxed)
    }

    pub fn start_tls(&self) {
        self.out.lock().unwrap().tls = Some(Tls::new());
    }

    /// Feeds handshake bytes from the phone. Returns the reply and whether the handshake just finished.
    pub fn handshake(&self, input: &[u8]) -> io::Result<(Vec<u8>, bool)> {
        let mut out = self.out.lock().unwrap();
        let tls = out.tls.as_mut().ok_or_else(no_tls)?;
        let was = tls.handshaking();
        let reply = tls.handshake(input)?;
        Ok((reply, was && !tls.handshaking()))
    }

    /// Appends the plaintext of one encrypted frame to `msg`.
    pub fn decrypt(&self, cipher: &[u8], msg: &mut Vec<u8>) -> io::Result<()> {
        self.out
            .lock()
            .unwrap()
            .tls
            .as_mut()
            .ok_or_else(no_tls)?
            .decrypt(cipher, msg)
    }

    pub fn pb(&self, ch: u8, id: u16, m: Pb) {
        self.send(ch, id, &m.0)
    }

    pub fn send(&self, ch: u8, id: u16, data: &[u8]) {
        let mut out = self.out.lock().unwrap();
        if !self.alive() || out.write(ch, id, data).is_err() {
            self.die();
        }
    }
}

impl Out {
    /// Like aasdk, only version/handshake/auth-complete (control ids 1-4) go in plain text.
    fn write(&mut self, ch: u8, id: u16, data: &[u8]) -> io::Result<()> {
        let enc = !(ch == CONTROL && id <= 4);
        let ctrl = ch != CONTROL && id == 8; // channel open response is a control message
        self.plain.clear();
        self.plain.extend_from_slice(&id.to_be_bytes());
        self.plain.extend_from_slice(data);
        let flags = F_FIRST | F_LAST | if enc { F_ENC } else { 0 } | if ctrl { F_CTRL } else { 0 };
        self.frame.clear();
        self.frame.extend_from_slice(&[ch, flags, 0, 0]);
        if enc {
            self.tls
                .as_mut()
                .ok_or_else(no_tls)?
                .encrypt(&self.plain, &mut self.frame)?;
        } else {
            self.frame.extend_from_slice(&self.plain);
        }
        let len = self.frame.len() - 4;
        // shortcut: no outgoing fragmentation; our messages are all small. Upgrade if one can exceed 16K.
        assert!(len <= 0x4000);
        (self.frame[2], self.frame[3]) = ((len >> 8) as u8, len as u8);
        self.usb.write_all(&self.frame)?;
        self.usb.submit(); // don't wait for the transfer: senders include the UI and audio threads
        Ok(())
    }
}

/// Blocking reads from the phone that give up once the session is dead.
pub struct UsbIn {
    pub ep: EndpointRead<Bulk>,
    pub conn: Arc<Conn>,
}

impl Read for UsbIn {
    fn read(&mut self, b: &mut [u8]) -> io::Result<usize> {
        loop {
            match self.ep.read(b) {
                Err(e) if e.kind() == io::ErrorKind::TimedOut && self.conn.alive() => continue,
                r => return r,
            }
        }
    }
}
