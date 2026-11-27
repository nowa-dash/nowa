//! Finds phones, switches them to Android Open Accessory mode and runs a session per connection.
use crate::Display;
use crate::conn::{self, Conn, UsbIn};
use nusb::descriptors::TransferType;
use nusb::transfer::{Bulk, ControlIn, ControlOut, ControlType, Direction, In, Out, Recipient};
use nusb::{DeviceInfo, MaybeFuture};
use std::sync::Arc;
use std::time::Duration;

const T: Duration = Duration::from_secs(1);

pub fn run(display: Arc<dyn Display>) {
    // shortcut: grows by one entry per distinct plugged device; fine for a car, never pruned
    let mut tried = Vec::new();
    loop {
        for d in nusb::list_devices().wait().into_iter().flatten() {
            if d.class() == 9 {
                continue; // hub
            }
            if d.vendor_id() == 0x18d1 && matches!(d.product_id(), 0x2d00 | 0x2d01) {
                if let Some(usb_in) = open(&d) {
                    connected(usb_in, &display);
                    break;
                }
                continue;
            }
            let key = (
                d.busnum(),
                d.device_address(),
                d.vendor_id(),
                d.product_id(),
            );
            if !tried.contains(&key) {
                tried.push(key);
                aoap_switch(&d);
            }
        }
        std::thread::sleep(T);
    }
}

fn connected(usb_in: UsbIn, display: &Arc<dyn Display>) {
    eprintln!("phone connected");
    *conn::CURRENT.lock().unwrap() = Some(usb_in.conn.clone());
    display.connected(true);
    crate::session::run(usb_in, display.clone());
    *conn::CURRENT.lock().unwrap() = None;
    display.connected(false);
}

/// Asks a phone to switch to accessory mode; it then re-enumerates as 18d1:2d00/2d01.
fn aoap_switch(d: &DeviceInfo) {
    let Ok(dev) = d.open().wait() else { return };
    let get = ControlIn {
        control_type: ControlType::Vendor,
        recipient: Recipient::Device,
        request: 51,
        value: 0,
        index: 0,
        length: 2,
    };
    if !matches!(dev.control_in(get, T).wait(), Ok(v) if v.len() == 2 && v[0] >= 1) {
        return;
    }
    let strings = [
        "Android",
        "Android Auto",
        "Android Auto",
        "2.0.1",
        "https://github.com/nowa-dash/nowa",
        "HU-AAAAAA001",
    ];
    let out = |request, index, data: &[u8]| {
        let c = ControlOut {
            control_type: ControlType::Vendor,
            recipient: Recipient::Device,
            request,
            value: 0,
            index,
            data,
        };
        let _ = dev.control_out(c, T).wait();
    };
    for (i, s) in strings.iter().enumerate() {
        out(52, i as u16, format!("{s}\0").as_bytes());
    }
    out(53, 0, &[]);
    eprintln!("switched device to accessory mode");
}

fn open(d: &DeviceInfo) -> Option<UsbIn> {
    let dev = d.open().wait().ok()?;
    let iface = dev.detach_and_claim_interface(0).wait().ok()?;
    let (mut ep_in, mut ep_out) = (0, 0);
    for e in iface
        .descriptor()?
        .endpoints()
        .filter(|e| e.transfer_type() == TransferType::Bulk)
    {
        if e.direction() == Direction::In {
            ep_in = e.address();
        } else {
            ep_out = e.address();
        }
    }
    let reader = iface
        .endpoint::<Bulk, In>(ep_in)
        .ok()?
        .reader(16384)
        .with_num_transfers(2)
        .with_read_timeout(Duration::from_millis(500));
    let writer = iface
        .endpoint::<Bulk, Out>(ep_out)
        .ok()?
        .writer(16384)
        .with_write_timeout(2 * T);
    Some(UsbIn {
        ep: reader,
        conn: Conn::new(writer),
    })
}
