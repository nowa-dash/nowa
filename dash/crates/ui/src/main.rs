use slint::ComponentHandle;
use std::error::Error;

slint::include_modules!();

mod aa;
mod music;
mod settings;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    aa::start(&ui);
    music::start(&ui);
    settings::start(&ui);
    ui.run()?;
    Ok(())
}
