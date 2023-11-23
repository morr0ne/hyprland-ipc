use hyprland_ipc::{EventListener, Result};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let mut event_listener = EventListener::new().await?;

    while let Some(event) = event_listener.try_next().await? {
        dbg!(event);
    }

    Ok(())
}
