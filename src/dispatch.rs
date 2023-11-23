use tokio::net::UnixStream;

use crate::Error;

pub struct Dispatcher {
    inner: UnixStream,
}

impl Dispatcher {
    pub async fn new() -> Result<Self, Error> {
        let his = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
            .map_err(|_err| Error::NoInstanceSignature)?;

        let socket = UnixStream::connect(format!("/tmp/hypr/{his}/.socket.sock")).await?;

        Ok(Self { inner: socket })
    }
}
