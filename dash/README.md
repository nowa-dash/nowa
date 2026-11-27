# dash

The car UI: one Slint app plus feature crates, all in `crates/`.

| Crate | What |
|---|---|
| `crates/ui` | Slint UI (`ui/`), glue per page in `src/` |
| `crates/android-auto` | wired Android Auto, no UI |
| `crates/music` | local music library and playback |

Build with `cargo build --release`. Runs under Wayland/X11 for development and directly on KMS in the car
(`SLINT_BACKEND=linuxkms`). `AA_RES=720|1080` raises the Android Auto resolution (default 480).

