//! Tiny protobuf encoder plus a varint field lookup; all the protocol needs.

#[derive(Default)]
pub struct Pb(pub Vec<u8>);

pub fn pb() -> Pb {
    Pb::default()
}

impl Pb {
    fn raw(&mut self, mut v: u64) {
        while v > 0x7f {
            self.0.push(v as u8 | 0x80);
            v >>= 7;
        }
        self.0.push(v as u8);
    }
    pub fn u(mut self, f: u32, v: u64) -> Self {
        self.raw((f as u64) << 3);
        self.raw(v);
        self
    }
    pub fn s(mut self, f: u32, d: &[u8]) -> Self {
        self.raw((f as u64) << 3 | 2);
        self.raw(d.len() as u64);
        self.0.extend_from_slice(d);
        self
    }
    pub fn str(self, f: u32, s: &str) -> Self {
        self.s(f, s.as_bytes())
    }
    pub fn m(self, f: u32, c: Pb) -> Self {
        self.s(f, &c.0)
    }
}

fn varint(p: &mut &[u8]) -> Option<u64> {
    let mut v = 0;
    for s in (0..64).step_by(7) {
        let (&c, rest) = p.split_first()?;
        *p = rest;
        v |= ((c & 0x7f) as u64) << s;
        if c & 0x80 == 0 {
            return Some(v);
        }
    }
    None
}

/// Varint value of `field`, or `def` if absent or malformed.
pub fn get(mut p: &[u8], field: u64, def: u64) -> u64 {
    while !p.is_empty() {
        let Some(k) = varint(&mut p) else { return def };
        let skip = match k & 7 {
            0 => match varint(&mut p) {
                Some(v) if k >> 3 == field => return v,
                Some(_) => 0,
                None => return def,
            },
            1 => 8,
            2 => match varint(&mut p) {
                Some(l) => l as usize,
                None => return def,
            },
            5 => 4,
            _ => return def,
        };
        if skip > p.len() {
            return def;
        }
        p = &p[skip..];
    }
    def
}
