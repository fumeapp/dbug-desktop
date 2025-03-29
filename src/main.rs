mod server;
mod gui;
mod storage;
mod settings;
use tokio::sync::watch;


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = watch::channel(());

    // Run server in background task
    tokio::spawn(server::listen(tx));

    // Run GUI on main thread
    gui::gui(rx)?;

    Ok(())
}
