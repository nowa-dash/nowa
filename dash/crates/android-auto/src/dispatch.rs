//! Handles messages on every channel except control: channel opens, sensors, input, audio, video, mic.
use crate::Display;
use crate::audio::{Mic, Speaker};
use crate::channel::*;
use crate::conn::Conn;
use crate::pb::{self, pb};
use crate::video::Decoder;
use std::sync::Arc;

pub struct Services {
    c: Arc<Conn>,
    display: Arc<dyn Display>,
    video: Option<Decoder>,
    speakers: [Option<Speaker>; COUNT],
    mic: Option<Mic>,
    session: [u64; COUNT],
}

impl Services {
    pub fn new(c: Arc<Conn>, display: Arc<dyn Display>) -> Self {
        Services {
            c,
            display,
            video: None,
            speakers: Default::default(),
            mic: None,
            session: [0; COUNT],
        }
    }

    pub fn handle(&mut self, ch: u8, p: &[u8]) {
        if p.len() < 2 {
            return;
        }
        let id = u16::from_be_bytes([p[0], p[1]]);
        let (p, i, c) = (&p[2..], ch as usize, &self.c);
        if ch == CONTROL {
            return crate::control::handle(c, id, p);
        }
        if id == 7 {
            // channel open request
            if (MEDIA..=SYSTEM).contains(&ch) && self.speakers[i].is_none() {
                self.speakers[i] = Speaker::new(ch);
            }
            return c.pb(ch, 8, pb().u(1, 0));
        }
        let focus = || c.pb(VIDEO, 0x8008, pb().u(1, 1).u(2, 0));
        match (ch, id) {
            (SENSOR, 0x8001) => {
                // sensor start: answer, then send the initial value (driving unrestricted / day)
                let t = pb::get(p, 1, 0);
                c.pb(ch, 0x8002, pb().u(1, 0));
                if t == 13 || t == 10 {
                    c.pb(ch, 0x8003, pb().m(t as u32, pb().u(1, 0)));
                }
            }
            (INPUT, 0x8002) => c.pb(ch, 0x8003, pb().u(1, 0)), // key binding request
            (INPUT | SENSOR, _) => {}
            (_, 0x0000 | 0x0001) => {
                // media: id 0 carries an 8 byte timestamp first, id 1 is codec config
                let d = if id == 0 {
                    p.get(8..).unwrap_or_default()
                } else {
                    p
                };
                // ack first, so the phone sends the next one while this one decodes
                c.pb(ch, 0x8004, pb().u(1, self.session[i]).u(2, 1));
                if ch == VIDEO {
                    self.video
                        .get_or_insert_with(|| Decoder::new(self.display.clone()))
                        .decode(d);
                } else if let Some(s) = &self.speakers[i] {
                    s.push(d);
                }
            }
            (_, 0x8000) => {
                c.pb(ch, 0x8003, pb().u(1, 2).u(2, 1).u(3, 0)); // setup: OK, max_unacked 1, config 0
                if ch == VIDEO {
                    focus();
                }
            }
            (VIDEO, 0x8007) => focus(),
            (_, 0x8001) => self.session[i] = pb::get(p, 1, 0), // start indication
            (MIC, 0x8005) => {
                let open = pb::get(p, 1, 0) != 0;
                match &self.mic {
                    Some(m) => m.set(open),
                    None if open => self.mic = Mic::new(c.clone()),
                    None => {}
                }
                c.pb(ch, 0x8006, pb().u(1, 0).u(2, 0));
            }
            _ => {}
        }
    }
}
