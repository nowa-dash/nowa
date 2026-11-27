# android-auto

Wired Android Auto head unit as a library: USB (`nusb`), TLS (`rustls`), the protocol (following aasdk/openauto),
H.264 decoding (V4L2 hardware decoder, OpenH264 fallback) and audio (PipeWire via its PulseAudio socket).
No UI: implement `Display` to receive frames, call `touch` to send input.

```rust
android_auto::start(android_auto::Config::new(480), Arc::new(MyDisplay));
android_auto::touch(0, android_auto::Touch::Down, x, y);
```

| File | What |
|---|---|
| `lib.rs` | public API |
| `usb.rs` | find phones, accessory mode, open endpoints |
| `conn.rs` | framing + encryption of outgoing messages, TLS state |
| `session.rs` | read loop: frames -> decrypted messages |
| `control.rs` | control channel (version, handshake, ping, focus, shutdown) |
| `discovery.rs` | what we tell the phone we support |
| `dispatch.rs` | all other channels (sensors, input, audio, video, mic) |
| `tls.rs`, `pb.rs` | rustls in memory, tiny protobuf |
| `audio/`, `video/` | speakers + mic, decoders + YUV conversion |
| `touch.rs`, `config.rs`, `channel.rs` | input, video settings, ids |

`AA_DECODER=sw` forces the software decoder. Hardware decoding needs access to `/dev/video*`.
