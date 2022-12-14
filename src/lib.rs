
use serde::{ser::Serializer, Serialize};
use tauri::{
  command,
  plugin::{Builder, TauriPlugin},
  AppHandle, Manager, Runtime, State, Window,
};
use std::{net::{TcpStream, SocketAddr, AddrParseError}, io::{Write, Read}, string::FromUtf8Error, time::Duration, str::FromStr};

use std::{sync::Mutex};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  FromUtf8(#[from] FromUtf8Error),
  #[error("Not connected.")]
  NotConnected,
  #[error("Already connected.")]
  AlreadyConnected,
  #[error(transparent)]
  AddrParse(#[from] AddrParseError),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

#[derive(Default)]
struct Connection(Mutex<Option<TcpStream>>);

#[command]
async fn connect<R: Runtime>(
  addr: String,
  _app: AppHandle<R>,
  _window: Window<R>,
  state: State<'_, Connection>,
) -> Result<()> {
  let mut connection = state.0.lock().unwrap();
  if let Some(_stream) = &mut *connection {
    // do nothing
    Err(Error::AlreadyConnected)
  } else {
    *connection = Some(TcpStream::connect(&SocketAddr::from_str(&addr)?)?);
    Ok(())
  }
}

#[command]
async fn connect_timeout<R: Runtime>(
  addr: String,
  timeout_ms: u64,
  _app: AppHandle<R>,
  _window: Window<R>,
  state: State<'_, Connection>,
) -> Result<()> {
  let mut connection = state.0.lock().unwrap();
  if let Some(_stream) = &mut *connection {
    // do nothing
    Err(Error::AlreadyConnected)
  } else {
    *connection = Some(TcpStream::connect_timeout(&SocketAddr::from_str(&addr)?, Duration::from_millis(timeout_ms))?);
    Ok(())
  }
}

#[command]
async fn disconnect<R: Runtime>(
  _app: AppHandle<R>,
  _window: Window<R>,
  state: State<'_, Connection>,
) -> Result<()> {
  let mut connection = state.0.lock().unwrap();
  if let Some(_stream) = &mut *connection {
    *connection = None;
    Ok(())
  } else {
    Err(Error::NotConnected)
  }
}

#[command]
async fn send_and_receive<R: Runtime>(
  request: String,
  _app: AppHandle<R>,
  _window: Window<R>,
  state: State<'_, Connection>,
) -> Result<String> {
  let mut connection = state.0.lock().unwrap();
  if let Some(stream) = &mut *connection {
    stream.write(request.as_bytes())?;

    let mut buf = [0u8; 1024];
    let size = stream.read(&mut buf)?;
    let response = String::from_utf8(buf[..size].to_vec())?;
    Ok(response)
  } else {
    Err(Error::NotConnected)
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("comm")
    .invoke_handler(tauri::generate_handler![connect, connect_timeout, disconnect, send_and_receive])
    .setup(|app| {
      app.manage(Connection::default());
      Ok(())
    })
    .build()
}
