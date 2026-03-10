//! Status bar management

use crate::config::Colorscheme;
use crate::workspace::Workspace;

#[derive(Debug, Clone)]
pub enum BarModule {
    Workspaces,
    Layout,
    Title,
    CPU,
    RAM,
    Disk,
    Battery,
    Brightness,
    Volume,
    WiFi,
    Clock,
    Tray,
}

pub struct BarConfig {
    pub position: BarPosition,
    pub height: u32,
    pub padding: u32,
    pub font: String,
    pub font_size: u32,
    pub colorscheme: Colorscheme,
    pub modules_left: Vec<BarModule>,
    pub modules_center: Vec<BarModule>,
    pub modules_right: Vec<BarModule>,
}

impl Default for BarConfig {
    fn default() -> Self {
        Self {
            position: BarPosition::Top,
            height: 24,
            padding: 5,
            font: "JetBrains Mono".to_string(),
            font_size: 12,
            colorscheme: Colorscheme::default(),
            modules_left: vec![BarModule::Workspaces],
            modules_center: vec![BarModule::Title],
            modules_right: vec![BarModule::CPU, BarModule::RAM, BarModule::Clock],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarPosition {
    Top,
    Bottom,
}

pub fn render_workspace_status(workspaces: &[Workspace], active: usize) -> String {
    let mut output = String::new();
    
    for (i, ws) in workspaces.iter().enumerate() {
        if !ws.windows.is_empty() || i == active {
            if i == active {
                output.push_str(&format!("[{}]", ws.name));
            } else {
                output.push_str(&format!(" {} ", ws.name));
            }
        }
    }
    
    output
}
