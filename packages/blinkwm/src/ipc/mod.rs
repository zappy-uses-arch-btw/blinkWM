use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write};
use tracing::info;
use std::sync::Arc;
use blinkwm_common::{IpcRequest, IpcResponse, WorkspaceInfo};

#[derive(Clone)]
pub struct IpcManager {
    listener: Arc<UnixListener>,
}

impl IpcManager {
    pub fn new() -> anyhow::Result<Self> {
        let uid = nix::unistd::getuid();
        let socket_path = format!("/tmp/blinkwm-{}.sock", uid);
        let _ = std::fs::remove_file(&socket_path);
        
        info!("Starting IPC listener at {}", socket_path);
        let listener = UnixListener::bind(&socket_path)?;
        listener.set_nonblocking(true)?;

        Ok(Self { listener: Arc::new(listener) })
    }

    pub fn accept(&self) -> Option<UnixStream> {
        self.listener.accept().ok().map(|(stream, _)| stream)
    }

    pub fn listener(&self) -> &UnixListener {
        &self.listener
    }
}

pub fn handle_ipc_client(mut stream: UnixStream, state: &mut crate::BlinkState) {
    let mut buffer = [0; 1024];
    if let Ok(n) = stream.read(&mut buffer) {
        if n > 0 {
            if let Ok(request) = serde_json::from_slice::<IpcRequest>(&buffer[..n]) {
                let response = match request {
                    IpcRequest::GetWorkspaces => {
                        let ws_info = state.workspaces.iter().enumerate().map(|(i, ws)| {
                            WorkspaceInfo {
                                id: ws.id,
                                active: i == state.active_workspace,
                                occupied: state.managed_windows.iter().any(|w| w.workspace == ws.id),
                            }
                        }).collect();
                        IpcResponse::Workspaces(ws_info)
                    }
                    IpcRequest::SwitchWorkspace(id) => {
                        if let Err(e) = crate::switch_workspace(state, id) {
                            IpcResponse::Error(e.to_string())
                        } else {
                            IpcResponse::Success
                        }
                    }
                };
                
                if let Ok(resp_json) = serde_json::to_vec(&response) {
                    let _ = stream.write_all(&resp_json);
                }
            }
        }
    }
}
