//! Control channel: version, TLS handshake, service discovery, ping, focus and shutdown.
use crate::channel::CONTROL;
use crate::conn::Conn;
use crate::pb::{self, pb};

pub fn handle(c: &Conn, id: u16, p: &[u8]) {
    match id {
        2 => {
            // version response -> start TLS
            c.start_tls();
            handshake(c, &[]);
        }
        3 => handshake(c, p),
        5 => crate::discovery::send(c),
        0x0b => c.pb(CONTROL, 0x0c, pb().u(1, pb::get(p, 1, 0))), // ping
        0x0d => c.pb(CONTROL, 0x0e, pb().u(1, 2)),                // navigation focus
        0x0f => {
            c.send(CONTROL, 0x10, &[]); // shutdown
            c.die()
        }
        // audio focus: release -> loss, anything else -> gain
        0x12 => c.pb(
            CONTROL,
            0x13,
            pb().u(1, if pb::get(p, 1, 0) == 4 { 3 } else { 1 }),
        ),
        _ => eprintln!("control msg {id:#x} ignored"),
    }
}

fn handshake(c: &Conn, input: &[u8]) {
    match c.handshake(input) {
        Ok((reply, done)) => {
            if !reply.is_empty() {
                c.send(CONTROL, 3, &reply);
            }
            if done {
                c.pb(CONTROL, 4, pb().u(1, 0)); // auth complete
            }
        }
        Err(e) => {
            eprintln!("TLS handshake failed: {e}");
            c.die()
        }
    }
}
