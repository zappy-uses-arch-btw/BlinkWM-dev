//! X11 connection - minimal implementation

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::rust_connection::RustConnection;
use log::info;
use std::sync::Arc;
use parking_lot::Mutex;

pub struct X11Connection {
    conn: Arc<Mutex<RustConnection>>,
    screen: Arc<Mutex<Screen>>,
    root: u32,
}

impl X11Connection {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (conn, screen_num) = RustConnection::connect(None)?;
        let setup = conn.setup();
        let screen = setup.roots.get(screen_num).cloned().ok_or("No screen")?;
        let root = screen.root;
        info!("Connected to X11, root: {}", root);
        
        Ok(Self { conn: Arc::new(Mutex::new(conn)), screen: Arc::new(Mutex::new(screen)), root })
    }
    
    pub fn wait_for_event(&self) -> Option<Event> { self.conn.lock().wait_for_event().ok() }
    pub fn poll_event(&self) -> Option<Event> { self.conn.lock().poll_for_event().ok().flatten() }
    
    pub fn set_input_focus(&self, window: u32) {
        use x11rb::protocol::xproto::Timestamp;
        let _ = self.conn.lock().set_input_focus(InputFocus::PARENT, window, 0u32);
    }
    
    pub fn send_close_window(&self, _window: u32) {}
    
    pub fn get_window_attributes(&self, window: u32) -> Option<GetWindowAttributesReply> {
        self.conn.lock().get_window_attributes(window).ok().and_then(|c| c.reply().ok())
    }
    
    pub fn get_window_geometry(&self, window: u32) -> Option<GetGeometryReply> {
        self.conn.lock().get_geometry(window).ok().and_then(|c| c.reply().ok())
    }
    
    pub fn get_window_class(&self, _window: u32) -> (String, String) { ("Unknown".to_string(), "unknown".to_string()) }
    pub fn get_window_name(&self, _window: u32) -> String { "Unknown".to_string() }
    
    pub fn reparent_window(&self, window: u32, parent: u32, x: i16, y: i16) -> Result<(), Box<dyn std::error::Error>> {
        self.conn.lock().reparent_window(window, parent, x, y)?;
        Ok(())
    }
    
    pub fn map_window(&self, window: u32) -> Result<(), Box<dyn std::error::Error>> {
        self.conn.lock().map_window(window)?;
        Ok(())
    }
    
    pub fn configure_window(&self, window: u32, x: Option<i32>, y: Option<i32>, width: Option<u32>, height: Option<u32>, border_width: Option<u32>, stack_mode: Option<StackMode>) -> Result<(), Box<dyn std::error::Error>> {
        let mut aux = ConfigureWindowAux::new();
        if let Some(v) = x { aux.x = Some(v); }
        if let Some(v) = y { aux.y = Some(v); }
        if let Some(v) = width { aux.width = Some(v); }
        if let Some(v) = height { aux.height = Some(v); }
        if let Some(v) = border_width { aux.border_width = Some(v); }
        if let Some(v) = stack_mode { aux.stack_mode = Some(v); }
        self.conn.lock().configure_window(window, &aux)?;
        Ok(())
    }
    
    pub fn root(&self) -> u32 { self.root }
    pub fn screen(&self) -> parking_lot::MutexGuard<'_, Screen> { self.screen.lock() }
    pub fn atoms(&self) -> &X11Atoms { &X11Atoms {} }
}

#[derive(Clone, Copy)]
pub struct X11Atoms {}

impl X11Atoms {}
