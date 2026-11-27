//! Touch input from the UI, in video pixel coordinates, sent to the phone.
use crate::channel::INPUT;
use crate::config::cfg;
use crate::conn::CURRENT;
use crate::pb::pb;
use std::sync::{LazyLock, Mutex};
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Touch {
    Down = 0,
    Up = 1,
    Move = 2,
}

const POINTERS: usize = 10;

struct State {
    on: [bool; POINTERS],
    pos: [(f32, f32); POINTERS],
    start: Instant,
}

static ST: LazyLock<Mutex<State>> = LazyLock::new(|| {
    Mutex::new(State {
        on: [false; POINTERS],
        pos: [(0.0, 0.0); POINTERS],
        start: Instant::now(),
    })
});

/// `pointer` is a finger id below 10 (use 0 for a single pointer). Moves and ups without a down are ignored.
pub fn touch(pointer: usize, action: Touch, x: f32, y: f32) {
    let mut st = ST.lock().unwrap();
    if pointer >= POINTERS || (action != Touch::Down && !st.on[pointer]) {
        return;
    }
    (st.on[pointer], st.pos[pointer]) = (true, (x, y));
    let (w, h) = (cfg().width, cfg().height);
    let mut t = pb();
    let (mut idx, mut n) = (0, 0);
    for i in (0..POINTERS).filter(|&i| st.on[i]) {
        if i == pointer {
            idx = n;
        }
        let (x, y) = st.pos[i];
        let loc = pb()
            .u(1, (x.max(0.0) as u32).min(w - 1) as u64)
            .u(2, (y.max(0.0) as u32).min(h - 1) as u64);
        t = t.m(1, loc.u(3, i as u64));
        n += 1;
    }
    if action == Touch::Up {
        st.on[pointer] = false;
    }
    // multi-touch uses Android MotionEvent codes: 5 = POINTER_DOWN, 6 = POINTER_UP
    let code = action as u64 + if n > 1 && action != Touch::Move { 5 } else { 0 };
    let ts = st.start.elapsed().as_micros() as u64;
    drop(st);
    let Some(c) = CURRENT.lock().unwrap().clone() else {
        return;
    };
    c.pb(INPUT, 0x8001, pb().u(1, ts).m(3, t.u(2, idx).u(3, code)));
}
