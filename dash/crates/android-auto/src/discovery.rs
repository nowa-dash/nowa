//! Service discovery response: tells the phone which channels and formats we support.
use crate::audio::FORMATS;
use crate::channel::*;
use crate::config::cfg;
use crate::conn::Conn;
use crate::pb::pb;

pub fn send(c: &Conn) {
    let cfg = cfg();
    let svc = |ch: u8, field, desc| pb().u(1, ch as u64).m(field, desc);
    let touch = pb().m(2, pb().u(1, cfg.width as u64).u(2, cfg.height as u64));
    // sensors: driving status + night mode (phone requires these)
    let sensors = pb().m(1, pb().u(1, 13)).m(1, pb().u(1, 10));
    let video_cfg = pb()
        .u(1, cfg.res())
        .u(2, 1 /* 30fps */)
        .u(3, 0)
        .u(4, 0)
        .u(5, cfg.dpi);
    let mut r = pb()
        .m(1, svc(INPUT, 4, touch))
        .m(1, svc(SENSOR, 2, sensors))
        .m(
            1,
            svc(VIDEO, 3, pb().u(1, 3 /* video */).m(4, video_cfg).u(5, 1)),
        );
    for (ch, kind, rate, chans) in FORMATS {
        let conf = pb().u(1, rate as u64).u(2, 16).u(3, chans as u64);
        r = r.m(
            1,
            match ch {
                MIC => svc(ch, 5, pb().u(1, 1).m(2, conf)),
                _ => svc(
                    ch,
                    3,
                    pb().u(1, 1 /* audio */).u(2, kind).m(3, conf).u(5, 1),
                ),
            },
        );
    }
    let r = r
        .str(2, "nowa")
        .str(3, "Universal")
        .str(4, "2026")
        .str(5, "0001")
        .u(6, 1);
    c.pb(
        CONTROL,
        6,
        r.str(7, "nowa")
            .str(8, "headunit")
            .str(9, "1")
            .str(10, "1.0")
            .u(11, 0),
    );
}
