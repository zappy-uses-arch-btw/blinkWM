use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IpcRequest {
    GetWorkspaces,
    SwitchWorkspace(usize),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceInfo {
    pub id: usize,
    pub active: bool,
    pub occupied: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IpcResponse {
    Workspaces(Vec<WorkspaceInfo>),
    Success,
    Error(String),
}
