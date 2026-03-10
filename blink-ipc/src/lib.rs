//! BlinkWM IPC Library
//! Shared communication types and utilities between BlinkWM components

use serde::{Deserialize, Serialize};

/// IPC message types for window manager communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcMessage {
    // Window operations
    WindowList,
    WindowMove { id: u32, x: i32, y: i32 },
    WindowResize { id: u32, width: u32, height: u32 },
    WindowClose { id: u32 },
    WindowFocus { id: u32 },
    WindowRaise { id: u32 },
    WindowLower { id: u32 },
    
    // Workspace operations
    WorkspaceSwitch { index: usize },
    WorkspaceMove { window_id: u32, workspace: usize },
    WorkspaceCreate { name: Option<String> },
    WorkspaceDelete { index: usize },
    WorkspaceRename { index: usize, name: String },
    
    // Layout operations
    LayoutSet { layout: String },
    LayoutNext,
    LayoutPrev,
    
    // Output operations
    OutputList,
    OutputEnable { name: String },
    OutputDisable { name: String },
    OutputPrimary { name: String },
    
    // System operations
    SystemInfo,
    Screenshot { mode: String },
    Clipboard { action: String },
    
    // Response
    Response(IpcResponse),
}

/// IPC response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcResponse {
    Ok,
    Err(String),
    WindowList(Vec<WindowInfo>),
    WorkspaceList(Vec<WorkspaceInfo>),
    OutputList(Vec<OutputInfo>),
    SystemInfo(SystemInfo),
    String(String),
}

/// Window information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub id: u32,
    pub title: String,
    pub class: String,
    pub workspace: usize,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub floating: bool,
    pub fullscreen: bool,
}

/// Workspace information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    pub index: usize,
    pub name: String,
    pub monitor: String,
    pub layout: String,
}

/// Output (monitor) information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputInfo {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub primary: bool,
}

/// System information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub kernel: String,
    pub cpu: String,
    pub ram_used: u64,
    pub ram_total: u64,
    pub disk_used: u64,
    pub disk_total: u64,
}

/// Layout types supported by BlinkWM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Layout {
    BSP,
    Grid,
    MainStack,
    Monocle,
    Spiral,
}

impl Layout {
    pub fn as_str(&self) -> &'static str {
        match self {
            Layout::BSP => "BSP",
            Layout::Grid => "Grid",
            Layout::MainStack => "MainStack",
            Layout::Monocle => "Monocle",
            Layout::Spiral => "Spiral",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "BSP" => Some(Layout::BSP),
            "GRID" => Some(Layout::Grid),
            "MAINSTACK" | "MAIN_STACK" => Some(Layout::MainStack),
            "MONOCLE" => Some(Layout::Monocle),
            "SPIRAL" => Some(Layout::Spiral),
            _ => None,
        }
    }
}

/// Window rule for per-app window configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowRule {
    pub class: String,
    pub instance: Option<String>,
    pub title: Option<String>,
    pub floating: bool,
    pub geometry: Option<Geometry>,
    pub opacity: Option<f32>,
    pub workspace: Option<usize>,
}

/// Window geometry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// Colorscheme configuration (Key=Value format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Colorscheme {
    pub borders: String,
    pub borders_active: String,
    pub background: String,
    pub surface: String,
    pub text: String,
    pub accent: String,
}

impl Default for Colorscheme {
    fn default() -> Self {
        Self {
            borders: "#3c3c3c".to_string(),
            borders_active: "#007ACC".to_string(),
            background: "#1e1e1e".to_string(),
            surface: "#252526".to_string(),
            text: "#cccccc".to_string(),
            accent: "#007ACC".to_string(),
        }
    }
}
