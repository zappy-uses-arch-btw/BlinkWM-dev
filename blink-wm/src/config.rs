//! Configuration management

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBinding {
    pub modifiers: Vec<String>,
    pub key: String,
    pub action: String,
    #[serde(default)]
    pub params: HashMap<String, String>,
}

impl KeyBinding {
    pub fn matches(&self, _keycode: u8, _state: u32) -> bool {
        false
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub startup_apps: Vec<String>,
    pub terminal: String,
    pub file_manager: String,
    pub menu: String,
    pub workspace_count: usize,
    pub workspace_names: Vec<String>,
    pub default_layout: String,
    pub layout_gaps_inner: i32,
    pub layout_gaps_outer: i32,
    pub smart_gaps: bool,
    pub smart_borders: bool,
    pub border_width: u32,
    pub border_active_width: u32,
    pub float_classes: Vec<String>,
    pub float_dialogs: bool,
    pub focus_follows_mouse: bool,
    pub sloppy_focus: bool,
    pub auto_focus: bool,
    pub scratchpad_auto_hide: bool,
    pub scratchpad_show_on_hotkey: bool,
    pub keybindings: Vec<KeyBinding>,
    pub colorscheme: Colorscheme,
    pub bar_enabled: bool,
    pub bar_position: String,
    pub bar_height: u32,
    pub bar_padding: u32,
    pub bar_modules_left: Vec<String>,
    pub bar_modules_center: Vec<String>,
    pub bar_modules_right: Vec<String>,
    pub ipc_socket_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Colorscheme {
    pub borders: String,
    pub borders_active: String,
    pub background: String,
    pub surface: String,
    pub text: String,
    pub text_dim: String,
    pub accent: String,
    pub urgent: String,
    pub bar_bg: String,
    pub bar_fg: String,
}

impl Default for Colorscheme {
    fn default() -> Self {
        Self {
            borders: "#3c3c3c".to_string(),
            borders_active: "#007ACC".to_string(),
            background: "#1e1e1e".to_string(),
            surface: "#252526".to_string(),
            text: "#cccccc".to_string(),
            text_dim: "#808080".to_string(),
            accent: "#007ACC".to_string(),
            urgent: "#FF5555".to_string(),
            bar_bg: "#1e1e1e".to_string(),
            bar_fg: "#cccccc".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            startup_apps: Vec::new(),
            terminal: "alacritty".to_string(),
            file_manager: "thunar".to_string(),
            menu: "blink-launch".to_string(),
            workspace_count: 10,
            workspace_names: (1..=10).map(|i| format!("{}", i)).collect(),
            default_layout: "BSP".to_string(),
            layout_gaps_inner: 10,
            layout_gaps_outer: 0,
            smart_gaps: true,
            smart_borders: true,
            border_width: 2,
            border_active_width: 2,
            float_classes: vec!["pavucontrol".to_string(), "nm-connection-editor".to_string(), "blueman-manager".to_string()],
            float_dialogs: true,
            focus_follows_mouse: false,
            sloppy_focus: true,
            auto_focus: false,
            scratchpad_auto_hide: true,
            scratchpad_show_on_hotkey: true,
            keybindings: vec![],
            colorscheme: Colorscheme::default(),
            bar_enabled: true,
            bar_position: "top".to_string(),
            bar_height: 24,
            bar_padding: 5,
            bar_modules_left: vec!["workspaces".to_string()],
            bar_modules_center: vec!["title".to_string()],
            bar_modules_right: vec!["cpu".to_string(), "ram".to_string(), "clock".to_string()],
            ipc_socket_path: "/tmp/blink-ipc.sock".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.join("blink-wm").join("config.json");
            if config_path.exists() {
                if let Ok(content) = fs::read_to_string(&config_path) {
                    if let Ok(config) = serde_json::from_str(&content) {
                        return config;
                    }
                }
            }
        }
        
        if PathBuf::from("/etc/blink-wm/config.json").exists() {
            if let Ok(content) = fs::read_to_string("/etc/blink-wm/config.json") {
                if let Ok(config) = serde_json::from_str(&content) {
                    return config;
                }
            }
        }
        
        Self::default()
    }
    
    pub fn save(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}
