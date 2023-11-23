use std::io::Error as IoError;
use tokio_util::codec::LinesCodecError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("")]
    MalformedInput,
    #[error("HYPRLAND_INSTANCE_SIGNATURE was not found. Are you sure Hyprland is running?")]
    NoInstanceSignature,
    #[error("")]
    UnknownEvent,
    #[error("")]
    LinesCodec(#[from] LinesCodecError),
    #[error("")]
    Io(#[from] IoError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
