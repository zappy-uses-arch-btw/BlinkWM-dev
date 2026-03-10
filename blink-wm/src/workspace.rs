//! Workspace management

use crate::window::Window;
use crate::layout::Layout;

#[derive(Debug, Clone)]
pub struct Workspace {
    pub index: usize,
    pub name: String,
    pub windows: Vec<Window>,
    pub layout: Layout,
    pub layout_gaps: Gaps,
    pub urgent: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Gaps {
    pub inner: i32,
    pub outer: i32,
    pub smart_gaps: bool,
}

impl Default for Gaps {
    fn default() -> Self {
        Self { inner: 10, outer: 0, smart_gaps: true }
    }
}

impl Workspace {
    pub fn new(index: usize, name: impl Into<String>) -> Self {
        Self {
            index,
            name: name.into(),
            windows: Vec::new(),
            layout: Layout::BSP,
            layout_gaps: Gaps::default(),
            urgent: false,
        }
    }
    
    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }
    
    pub fn remove_window(&mut self, window_id: u32) -> Option<Window> {
        if let Some(pos) = self.windows.iter().position(|w| w.id == window_id) {
            Some(self.windows.remove(pos))
        } else {
            None
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.windows.len()
    }
}

pub struct WorkspaceManager {
    workspaces: Vec<Workspace>,
    active: usize,
}

impl WorkspaceManager {
    pub fn new() -> Self {
        let workspaces = (0..10).map(|i| Workspace::new(i, format!("{}", i + 1))).collect();
        Self { workspaces, active: 0 }
    }
    
    pub fn active(&self) -> &Workspace {
        &self.workspaces[self.active]
    }
    
    pub fn active_mut(&mut self) -> &mut Workspace {
        &mut self.workspaces[self.active]
    }
    
    pub fn switch_to(&mut self, index: usize) {
        if index < self.workspaces.len() {
            self.active = index;
        }
    }
}

impl Default for WorkspaceManager {
    fn default() -> Self {
        Self::new()
    }
}
