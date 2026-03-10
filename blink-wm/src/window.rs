//! Window management - simplified

use crate::x11::X11Connection;
use parking_lot::RwLock;

#[derive(Debug, Clone)]
pub struct Window {
    pub id: u32,
    pub title: String,
    pub class: String,
    pub instance: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub border_width: u32,
    pub floating: bool,
    pub fullscreen: bool,
    pub focused: bool,
    pub marked: bool,
    pub workspace: usize,
    pub scratchpad: bool,
    pub transient_for: Option<u32>,
    pub window_type: WindowType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowType { Normal, Dialog, Splash, Utility, Menu, Toolbar, Dock, Notification, Unknown }

impl Window {
    pub fn new(id: u32) -> Self {
        Self { id, title: String::new(), class: String::new(), instance: String::new(), x: 0, y: 0, width: 100, height: 100, border_width: 1, floating: false, fullscreen: false, focused: false, marked: false, workspace: 0, scratchpad: false, transient_for: None, window_type: WindowType::Normal }
    }
    pub fn should_tile(&self) -> bool { !self.floating && !self.fullscreen && self.window_type == WindowType::Normal && self.transient_for.is_none() }
    pub fn is_dialog(&self) -> bool { matches!(self.window_type, WindowType::Dialog | WindowType::Splash | WindowType::Utility) }
}

pub struct WindowManager {
    needs_reload: RwLock<bool>,
    float_classes: RwLock<Vec<String>>,
}

impl WindowManager {
    pub fn new(_x11: &X11Connection) -> Self {
        Self { needs_reload: RwLock::new(false), float_classes: RwLock::new(vec!["pavucontrol".to_string(), "nm-connection-editor".to_string()]) }
    }
    
    pub fn handle_map_request(&self, x11: &X11Connection, window: u32) {
        if let Some(attrs) = x11.get_window_attributes(window) {
            if attrs.override_redirect { return; }
            
            let geometry = x11.get_window_geometry(window);
            let (class, instance) = x11.get_window_class(window);
            let title = x11.get_window_name(window);
            
            let mut window = Window::new(window);
            window.title = title;
            window.class = class;
            window.instance = instance;
            window.window_type = WindowType::Normal;
            
            if let Some(geo) = geometry {
                window.x = geo.x as i32;
                window.y = geo.y as i32;
                window.width = geo.width as u32;
                window.height = geo.height as u32;
            }
            
            let float_classes = self.float_classes.read();
            if float_classes.iter().any(|c| c.eq_ignore_ascii_case(&window.class)) { window.floating = true; }
            if window.is_dialog() { window.floating = true; }
        }
        
        let _ = x11.reparent_window(window, x11.root(), 0, 0);
        let _ = x11.map_window(window);
        self.set_needs_reload();
    }
    
    pub fn handle_unmap_notify(&self, _x11: &X11Connection, _window: u32) { self.set_needs_reload(); }
    pub fn handle_destroy_notify(&self, _x11: &X11Connection, _window: u32) { self.set_needs_reload(); }
    
    pub fn handle_configure_request(&self, x11: &X11Connection, event: x11rb::protocol::xproto::ConfigureRequestEvent) {
        let _ = x11.configure_window(event.window, Some(event.x as i32), Some(event.y as i32), Some(event.width as u32), Some(event.height as u32), Some(event.border_width as u32), None);
    }
    
    pub fn set_needs_reload(&self) { *self.needs_reload.write() = true; }
    pub fn needs_reload(&self) -> bool { *self.needs_reload.read() }
    pub fn clear_reload_flag(&self) { *self.needs_reload.write() = false; }
}
