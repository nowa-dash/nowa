//! Hardware decoding through a V4L2 stateful (memory-to-memory) decoder, e.g. bcm2835-codec on the Pi.
//! The decoder lives on its own thread; decoded frames arrive on v4l2r's capture thread.
use super::convert::{Yuv, show};
use crate::Display;
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::sync::{Arc, Mutex};
use v4l2r::decoder::stateful::Decoder;
use v4l2r::decoder::{DecoderEvent, FormatChangedReply};
use v4l2r::device::queue::direction::Capture;
use v4l2r::device::queue::dqbuf::DqBuffer;
use v4l2r::device::queue::handles_provider::MmapProvider;
use v4l2r::memory::{MemoryType, MmapHandle};
use v4l2r::{Format, PixelFormat, PlaneLayout, Rect};

/// Starts the decoder thread if a hardware H.264 decoder exists.
pub fn spawn(display: Arc<dyn Display>) -> Option<SyncSender<Vec<u8>>> {
    let (tx, rx) = sync_channel(4);
    let (ok_tx, ok_rx) = sync_channel(1);
    std::thread::spawn(move || run(rx, ok_tx, display));
    ok_rx.recv().unwrap_or(false).then_some(tx)
}

type Fmt = Arc<Mutex<Option<(Format, Rect)>>>;

fn run(rx: Receiver<Vec<u8>>, ok: SyncSender<bool>, display: Arc<dyn Display>) {
    let fmt: Fmt = Arc::default();
    let start = |path: String| {
        let (f1, f2, d) = (fmt.clone(), fmt.clone(), display.clone());
        let on_event = move |ev: DecoderEvent<MmapProvider>| {
            if let DecoderEvent::FrameDecoded(dq) = ev {
                frame(&*d, &f1, dq);
            }
        };
        let on_format = move |f: v4l2r::device::queue::FormatBuilder, visible: Rect, min: usize| {
            let format = f.format();
            *f2.lock().unwrap() = Some((format.clone(), visible));
            Ok(FormatChangedReply {
                provider: MmapProvider::new(format),
                mem_type: MemoryType::Mmap,
                num_buffers: min,
            })
        };
        let h264: PixelFormat = b"H264".into();
        Decoder::open(path.as_ref())
            .ok()?
            .set_output_format(|f| {
                let f: Format = f
                    .set_pixelformat(h264)
                    .set_planes_layout(vec![PlaneLayout {
                        sizeimage: 1 << 20,
                        ..Default::default()
                    }])
                    .apply()?;
                anyhow::ensure!(f.pixelformat == h264, "no H.264 decoding");
                Ok(())
            })
            .ok()?
            .allocate_output_buffers::<Vec<MmapHandle>>(4)
            .ok()?
            .start(|_| (), on_event, on_format)
            .ok()
            .inspect(|_| eprintln!("video: hardware decoder {path}"))
    };
    let Some(mut dec) = (0..64).find_map(|i| start(format!("/dev/video{i}"))) else {
        let _ = ok.send(false);
        return;
    };
    let _ = ok.send(true);
    for h264 in rx {
        let Ok(buf) = dec.get_buffer() else { break };
        {
            let Some(mut map) = buf.get_plane_mapping(0) else {
                break;
            };
            let Some(dst) = map.get_mut(..h264.len()) else {
                continue;
            }; // larger than the 1MB input buffer
            dst.copy_from_slice(&h264);
        }
        if buf.queue(&[h264.len()]).is_err() {
            break;
        }
    }
    let _ = dec.stop();
}

fn frame(display: &dyn Display, fmt: &Fmt, dq: DqBuffer<Capture, Vec<MmapHandle>>) {
    let Some((f, r)) = fmt.lock().unwrap().clone() else {
        return;
    };
    let maps: Vec<_> = (0..dq.data.num_planes())
        .filter_map(|i| dq.get_plane_mapping(i))
        .collect();
    let Some(first) = maps.first().filter(|m| !m.is_empty()) else {
        return;
    };
    let stride = |i: usize| f.plane_fmt[i].bytesperline as usize;
    let (s, ysize) = (stride(0), stride(0) * f.height as usize);
    // shortcut: assumes the visible rect starts at 0,0 (true for phone streams); crop offsets are ignored
    let fourcc = f.pixelformat.to_fourcc();
    let yuv = match (&fourcc, maps.len()) {
        (b"NV12", 1) => first
            .split_at_checked(ysize)
            .map(|(y, uv)| Yuv::Nv12([(y, s), (uv, s)])),
        (b"NM12", 2) => Some(Yuv::Nv12([(first, s), (&maps[1], stride(1))])),
        (b"YU12", 1) => first.split_at_checked(ysize).and_then(|(y, rest)| {
            let (u, v) = rest.split_at_checked(s / 2 * f.height as usize / 2)?;
            Some(Yuv::I420([(y, s), (u, s / 2), (v, s / 2)]))
        }),
        (b"YM12", 3) => Some(Yuv::I420([
            (first, s),
            (&maps[1], stride(1)),
            (&maps[2], stride(2)),
        ])),
        _ => None,
    };
    match yuv {
        Some(yuv) => show(display, r.width, r.height, yuv),
        None => eprintln!("video: unsupported decoder output {:?}", f.pixelformat),
    }
}
