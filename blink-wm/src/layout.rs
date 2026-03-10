//! Tiling layout algorithms - simplified

use crate::workspace::{Workspace, Gaps};
use crate::x11::X11Connection;
use crate::window::Window;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout { BSP, Grid, MainStack, Monocle, Spiral }

impl Layout {
    pub fn as_str(&self) -> &'static str {
        match self { Layout::BSP => "BSP", Layout::Grid => "Grid", Layout::MainStack => "MainStack", Layout::Monocle => "Monocle", Layout::Spiral => "Spiral" }
    }
}

pub struct LayoutManager {
    pub border_width: u32,
    pub border_active: u32,
    pub smart_gaps: bool,
    pub smart_borders: bool,
}

impl LayoutManager {
    pub fn new() -> Self { Self { border_width: 2, border_active: 2, smart_gaps: true, smart_borders: true } }
    
    pub fn tile_windows(&self, x11: &X11Connection, workspace: &Workspace, layout: &Layout) {
        let windows: Vec<_> = workspace.windows.iter().filter(|w| w.should_tile()).collect();
        if windows.is_empty() { return; }
        
        let gaps = if self.smart_gaps && windows.len() == 1 { Gaps { inner: 0, outer: 0, smart_gaps: true } } else { workspace.layout_gaps };
        
        let screen = x11.screen();
        let width = screen.width_in_pixels;
        let height = screen.height_in_pixels;
        
        let x = gaps.outer as u32;
        let y = gaps.outer as u32;
        let w = (width as i32 - 2 * gaps.outer) as u32;
        let h = (height as i32 - 2 * gaps.outer - 24) as u32;
        
        match layout {
            Layout::BSP => self.tile_bsp(x11, &windows, x as i32, y as i32, w, h, gaps),
            Layout::Grid => self.tile_grid(x11, &windows, x as i32, y as i32, w, h, gaps),
            Layout::MainStack => self.tile_main_stack(x11, &windows, x as i32, y as i32, w, h, gaps),
            Layout::Monocle => self.tile_monocle(x11, &windows, x as i32, y as i32, w, h),
            Layout::Spiral => self.tile_spiral(x11, &windows, x as i32, y as i32, w, h, gaps),
        }
    }
    
    fn tile_bsp(&self, x11: &X11Connection, windows: &[&Window], x: i32, y: i32, width: u32, height: u32, gaps: Gaps) {
        if windows.is_empty() { return; }
        if windows.len() == 1 { let _ = x11.configure_window(windows[0].id, Some(x), Some(y), Some(width), Some(height), Some(self.border_width), None); return; }
        
        for (i, window) in windows.iter().enumerate() {
            let split_vertical = i % 2 == 0;
            let ratio = 0.5;
            let (wx, wy, ww, wh) = if split_vertical {
                let w = (width as f32 * ratio) as u32;
                (x + (i as i32 / 2) * w as i32, y, w, height)
            } else {
                let h = (height as f32 * ratio) as u32;
                (x, y + (i as i32 / 2) * h as i32, width, h)
            };
            let _ = x11.configure_window(window.id, Some(wx + gaps.inner), Some(wy + gaps.inner), Some(ww.saturating_sub(2 * gaps.inner as u32)), Some(wh.saturating_sub(2 * gaps.inner as u32)), Some(self.border_width), None);
        }
    }
    
    fn tile_grid(&self, x11: &X11Connection, windows: &[&Window], x: i32, y: i32, width: u32, height: u32, gaps: Gaps) {
        if windows.is_empty() { return; }
        let n = windows.len() as f32;
        let cols = n.sqrt().ceil() as u32;
        let rows = (n / cols as f32).ceil() as u32;
        let cell_width = width / cols;
        let cell_height = height / rows;
        
        for (i, window) in windows.iter().enumerate() {
            let col = (i as u32) % cols;
            let row = (i as u32) / cols;
            let wx = x + col as i32 * cell_width as i32;
            let wy = y + row as i32 * cell_height as i32;
            let _ = x11.configure_window(window.id, Some(wx + gaps.inner), Some(wy + gaps.inner), Some(cell_width.saturating_sub(2 * gaps.inner as u32)), Some(cell_height.saturating_sub(2 * gaps.inner as u32)), Some(self.border_width), None);
        }
    }
    
    fn tile_main_stack(&self, x11: &X11Connection, windows: &[&Window], x: i32, y: i32, width: u32, height: u32, gaps: Gaps) {
        if windows.is_empty() { return; }
        if windows.len() == 1 { let _ = x11.configure_window(windows[0].id, Some(x), Some(y), Some(width), Some(height), Some(self.border_width), None); return; }
        
        let main_width = width / 2;
        let stack_x = x + main_width as i32;
        let _ = x11.configure_window(windows[0].id, Some(x + gaps.inner), Some(y + gaps.inner), Some(main_width.saturating_sub(2 * gaps.inner as u32)), Some(height.saturating_sub(2 * gaps.inner as u32)), Some(self.border_width), None);
        
        let stack_height = height / (windows.len() - 1) as u32;
        for (i, window) in windows.iter().skip(1).enumerate() {
            let wy = y + i as i32 * stack_height as i32;
            let _ = x11.configure_window(window.id, Some(stack_x + gaps.inner), Some(wy + gaps.inner), Some(width.saturating_sub(main_width).saturating_sub(2 * gaps.inner as u32)), Some(stack_height.saturating_sub(2 * gaps.inner as u32)), Some(self.border_width), None);
        }
    }
    
    fn tile_monocle(&self, x11: &X11Connection, windows: &[&Window], x: i32, y: i32, width: u32, height: u32) {
        for window in windows { let _ = x11.configure_window(window.id, Some(x), Some(y), Some(width), Some(height), Some(self.border_width), None); }
    }
    
    fn tile_spiral(&self, x11: &X11Connection, windows: &[&Window], x: i32, y: i32, width: u32, height: u32, gaps: Gaps) {
        if windows.is_empty() { return; }
        if windows.len() == 1 { let _ = x11.configure_window(windows[0].id, Some(x), Some(y), Some(width), Some(height), Some(self.border_width), None); return; }
        
        let mut current_x = x; let mut current_y = y; let mut current_w = width; let mut current_h = height;
        
        for (i, window) in windows.iter().enumerate() {
            let split_ratio = if i % 4 < 2 { 0.6 } else { 0.4 };
            let (wx, wy, ww, wh) = match i % 4 {
                0 => { let w = (current_w as f32 * split_ratio) as u32; let r = (current_x, current_y, w, current_h); current_x += w as i32; current_w = current_w.saturating_sub(w); r }
                1 => { let h = (current_h as f32 * split_ratio) as u32; let r = (current_x, current_y, current_w, h); current_y += h as i32; current_h = current_h.saturating_sub(h); r }
                2 => { let w = (current_w as f32 * (1.0 - split_ratio)) as u32; current_x += w as i32; current_w = current_w.saturating_sub(w); (current_x, current_y, current_w, current_h) }
                _ => { let h = (current_h as f32 * (1.0 - split_ratio)) as u32; current_y += h as i32; current_h = current_h.saturating_sub(h); (current_x, current_y, current_w, current_h) }
            };
            let _ = x11.configure_window(window.id, Some(wx + gaps.inner), Some(wy + gaps.inner), Some(ww.saturating_sub(2 * gaps.inner as u32)), Some(wh.saturating_sub(2 * gaps.inner as u32)), Some(self.border_width), None);
        }
    }
}

impl Default for LayoutManager { fn default() -> Self { Self::new() } }
