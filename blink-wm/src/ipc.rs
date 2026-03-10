//! IPC server

use std::io::{Read, Write};
use std::os::unix::net::UnixListener;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcMessage {
    WindowList,
    WindowClose { id: u32 },
    WorkspaceSwitch { index: usize },
    LayoutSet { layout: String },
    SystemInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcResponse {
    Ok,
    Err(String),
    String(String),
}

pub struct IpcServer {
    listener: UnixListener,
    socket_path: PathBuf,
}

impl IpcServer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let socket_path = PathBuf::from("/tmp/blink-ipc.sock");
        
        if socket_path.exists() {
            std::fs::remove_file(&socket_path)?;
        }
        
        let listener = UnixListener::bind(&socket_path)?;
        listener.set_nonblocking(true)?;
        
        log::info!("IPC server listening on {:?}", socket_path);
        
        Ok(Self { listener, socket_path })
    }
    
    pub fn try_recv(&self) -> Option<IpcMessage> {
        match self.listener.accept() {
            Ok((mut stream, _)) => {
                let mut buffer = Vec::new();
                let mut buf = [0u8; 4096];
                
                match stream.read(&mut buf) {
                    Ok(0) => None,
                    Ok(n) => {
                        buffer.extend_from_slice(&buf[..n]);
                        serde_json::from_slice(&buffer).ok()
                    }
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }
    
    pub fn send_response(&self, stream: &mut std::os::unix::net::UnixStream, response: &IpcResponse) {
        if let Ok(data) = serde_json::to_vec(response) {
            let _ = stream.write_all(&data);
        }
    }
}

impl Drop for IpcServer {
    fn drop(&mut self) {
        if self.socket_path.exists() {
            let _ = std::fs::remove_file(&self.socket_path);
        }
    }
}
