use slint::ComponentHandle;
use std::process::Command;

use crate::AppWindow;

pub fn start(ui: &AppWindow) {
    ui.set_app_version(env!("CARGO_PKG_VERSION").into());

    ui.on_shutdown(
        move || match Command::new("systemctl").arg("poweroff").spawn() {
            Ok(_) => println!("Shutting down, bye!"),
            Err(error) => eprintln!("Failed to shut down: {error}"),
        },
    );

    ui.on_reboot(
        move || match Command::new("systemctl").arg("reboot").spawn() {
            Ok(_) => println!("Rebooting, see you soon!"),
            Err(error) => eprintln!("Failed to reboot: {error}"),
        },
    );

    let ui_handle = ui.as_weak();
    ui.on_toggle_dark_mode(move || {
        let ui = ui_handle.unwrap();
        ui.set_dark_mode(!ui.get_dark_mode());
        // TODO: store theme in appsettings somewhere
    });
}
