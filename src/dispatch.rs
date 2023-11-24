use std::num::NonZeroI32;

use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
};

use crate::{params::Window, Error};

pub struct Dispatcher {
    socket_path: String,
}

impl Dispatcher {
    pub async fn new() -> Result<Self, Error> {
        let his = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
            .map_err(|_err| Error::NoInstanceSignature)?;

        Ok(Self {
            socket_path: format!("/tmp/hypr/{his}/.socket.sock"),
        })
    }

    async fn call_command(&mut self, command: &str) -> Result<String, Error> {
        let mut socket = UnixStream::connect(&self.socket_path).await?;

        socket.write_all(command.as_bytes()).await?;

        let mut response = String::new();

        socket.read_to_string(&mut response).await?;

        if response == "ok" {
            Ok(response)
        } else {
            Err(Error::NotOkResponse(response))
        }
    }

    pub async fn clients(&mut self) -> Result<Vec<Client>, Error> {
        let response = self.call_command("j/clients").await?;

        serde_json::from_str(&response).map_err(|_err| Error::MalformedInput) // TODO: handle the serde error properly
    }

    pub async fn toggle_floating(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub async fn pin(&mut self, window: Option<Window>) -> Result<String, Error> {
        if let Some(window) = window {
            self.call_command(&format!("/dispatch pin {window}")).await
        } else {
            self.call_command("/dispatch pin").await
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
    pub address: String,
    pub mapped: bool,
    pub hidden: bool,
    pub at: [u32; 2],
    pub size: [u32; 2],
    pub workspace: Workspace,
    pub floating: bool,
    pub monitor: i64,
    pub class: String,
    pub title: String,
    #[serde(rename = "initialClass")]
    pub initial_class: String,
    #[serde(rename = "initialTitle")]
    pub initial_title: String,
    pub pid: NonZeroI32,
    pub xwayland: bool,
    pub pinned: bool,
    pub fullscreen: bool,
    #[serde(rename = "fullscreenMode")]
    pub fullscreen_mode: i64,
    #[serde(rename = "fakeFullscreen")]
    pub fake_fullscreen: bool,
    pub grouped: Vec<String>,
    pub swallowing: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Workspace {
    pub id: i64,
    pub name: String,
}
