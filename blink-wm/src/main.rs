//! BlinkWM - minimal window manager

mod x11;
mod window;
mod workspace;
mod layout;
mod config;
mod ipc;
mod bar;

use std::sync::Arc;
use parking_lot::RwLock;
use once_cell::sync::Lazy;
use log::info;

use x11::X11Connection;
use window::WindowManager;
use workspace::WorkspaceManager;
use layout::{Layout, LayoutManager};
use config::Config;

static WM_STATE: Lazy<Arc<RwLock<WmState>>> = Lazy::new(|| Arc::new(RwLock::new(WmState::default())));

#[derive(Default)]
pub struct WmState {
    pub windows: std::collections::HashMap<u32, window::Window>,
    pub workspaces: Vec<workspace::Workspace>,
    pub active_workspace: usize,
    pub layouts: Vec<Layout>,
    pub current_layout: usize,
    pub config: config::Config,
    pub running: bool,
}

impl WmState {
    pub fn new() -> Self {
        let mut state = Self::default();
        state.config = Config::load();
        state.workspaces = (0..10).map(|i| workspace::Workspace::new(i, format!("{}", i + 1))).collect();
        state.layouts = vec![Layout::BSP, Layout::Grid, Layout::MainStack, Layout::Monocle, Layout::Spiral];
        state.current_layout = 0;
        state.active_workspace = 0;
        state.running = true;
        state
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).format_timestamp_millis().init();
    info!("Starting BlinkWM v{}", env!("CARGO_PKG_VERSION"));
    
    let config = Config::load();
    info!("Configuration loaded");
    
    let x11 = X11Connection::new()?;
    info!("X11 connection established");
    
    let window_manager = WindowManager::new(&x11);
    let workspace_manager = WorkspaceManager::new();
    let layout_manager = LayoutManager::new();
    
    info!("Entering main event loop");
    
    while WM_STATE.read().running {
        if let Some(event) = x11.wait_for_event() {
            match event {
                x11rb::protocol::Event::MapRequest(e) => window_manager.handle_map_request(&x11, e.window),
                x11rb::protocol::Event::UnmapNotify(e) => window_manager.handle_unmap_notify(&x11, e.window),
                x11rb::protocol::Event::DestroyNotify(e) => window_manager.handle_destroy_notify(&x11, e.window),
                x11rb::protocol::Event::ConfigureRequest(e) => window_manager.handle_configure_request(&x11, e),
                _ => {}
            }
        }
        
        if window_manager.needs_reload() {
            let ws = &WM_STATE.read().workspaces[WM_STATE.read().active_workspace];
            let layout = &WM_STATE.read().layouts[WM_STATE.read().current_layout];
            layout_manager.tile_windows(&x11, ws, layout);
            window_manager.clear_reload_flag();
        }
    }
    
    info!("BlinkWM shutting down");
    Ok(())
}
