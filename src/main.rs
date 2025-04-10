mod components;
mod gui;
mod server;
mod settings;
mod storage;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    gui::gui()?;
    Ok(())
}
