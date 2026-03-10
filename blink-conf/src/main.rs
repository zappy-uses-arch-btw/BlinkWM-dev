//! BlinkWM Config Tool
//! A TUI for configuring BlinkWM settings

use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, List, ListItem},
    layout::{Layout, Constraint, Direction, Rect, Alignment},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;

/// Application state
struct App {
    // Navigation
    current_section: Section,
    selected_item: usize,
    
    // Settings
    settings: Settings,
    
    // Input mode
    quit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    System,
    Keybindings,
    Theming,
    Windows,
    Workspaces,
    Layout,
    Bar,
    Autostart,
    Picom,
    Wallpaper,
    Profiles,
}

impl Section {
    fn as_str(&self) -> &'static str {
        match self {
            Section::System => "System",
            Section::Keybindings => "Keybindings",
            Section::Theming => "Theming",
            Section::Windows => "Windows",
            Section::Workspaces => "Workspaces",
            Section::Layout => "Layout",
            Section::Bar => "Bar",
            Section::Autostart => "Autostart",
            Section::Picom => "Picom",
            Section::Wallpaper => "Wallpaper",
            Section::Profiles => "Profiles",
        }
    }
    
    fn all() -> Vec<Section> {
        vec![
            Section::System,
            Section::Keybindings,
            Section::Theming,
            Section::Windows,
            Section::Workspaces,
            Section::Layout,
            Section::Bar,
            Section::Autostart,
            Section::Picom,
            Section::Wallpaper,
            Section::Profiles,
        ]
    }
}

/// Settings
#[derive(Debug, Clone)]
struct Settings {
    terminal: String,
    file_manager: String,
    border_width: u32,
    gap_size: u32,
    smart_gaps: bool,
    smart_borders: bool,
    sloppy_focus: bool,
    os: String,
    kernel: String,
    cpu: String,
    ram_used: u64,
    ram_total: u64,
    disk_used: u64,
    disk_total: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            terminal: "alacritty".to_string(),
            file_manager: "thunar".to_string(),
            border_width: 2,
            gap_size: 10,
            smart_gaps: true,
            smart_borders: true,
            sloppy_focus: true,
            os: "Arch Linux".to_string(),
            kernel: "6.x.x".to_string(),
            cpu: "Ryzen 7 5800X".to_string(),
            ram_used: 16384,
            ram_total: 32768,
            disk_used: 128,
            disk_total: 256,
        }
    }
}

impl App {
    fn new() -> Self {
        Self {
            current_section: Section::System,
            selected_item: 0,
            settings: Settings::default(),
            quit: false,
        }
    }
    
    /// Move selection up
    fn move_up(&mut self) {
        if self.selected_item > 0 {
            self.selected_item -= 1;
        }
    }
    
    /// Move selection down
    fn move_down(&mut self) {
        let items = match self.current_section {
            Section::System => 1,
            Section::Keybindings => 20,
            Section::Theming => 10,
            Section::Windows => 5,
            Section::Workspaces => 10,
            Section::Layout => 5,
            Section::Bar => 10,
            Section::Autostart => 10,
            Section::Picom => 10,
            Section::Wallpaper => 5,
            Section::Profiles => 5,
        };
        
        if self.selected_item < items {
            self.selected_item += 1;
        }
    }
    
    /// Move to next section
    fn next_section(&mut self) {
        let sections = Section::all();
        if let Some(pos) = sections.iter().position(|s| *s == self.current_section) {
            if pos < sections.len() - 1 {
                self.current_section = sections[pos + 1];
                self.selected_item = 0;
            }
        }
    }
    
    /// Move to previous section
    fn prev_section(&mut self) {
        let sections = Section::all();
        if let Some(pos) = sections.iter().position(|s| *s == self.current_section) {
            if pos > 0 {
                self.current_section = sections[pos - 1];
                self.selected_item = 0;
            }
        }
    }
}

/// Run the config tool
fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Create app
    let mut app = App::new();
    
    // Main loop
    loop {
        // Draw
        terminal.draw(|f| ui(f, &app))?;
        
        // Handle events
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        app.move_up();
                    }
                    KeyCode::Down => {
                        app.move_down();
                    }
                    KeyCode::Left => {
                        app.prev_section();
                    }
                    KeyCode::Right => {
                        app.next_section();
                    }
                    KeyCode::Enter => {
                        // Handle enter (apply/save/etc)
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        
        if app.quit {
            break;
        }
    }
    
    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    Ok(())
}

/// Render UI
fn ui(frame: &mut Frame, app: &App) {
    let area = frame.size();
    
    // Layout: sidebar + content
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20),
            Constraint::Min(20),
        ])
        .split(area);
    
    // Sidebar
    let sidebar = chunks[0];
    let sidebar_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(1),   // Items
        ])
        .split(sidebar);
    
    // Title
    let title = Paragraph::new(" blink-conf ")
        .style(Style::default().fg(Color::LightBlue))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    frame.render_widget(title, sidebar_chunks[0]);
    
    // Section list
    let sections = Section::all();
    let items: Vec<ListItem> = sections
        .iter()
        .map(|s| {
            let style = if *s == app.current_section {
                Style::default().fg(Color::LightBlue).bg(Color::DarkGray)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!(" {}", s.as_str())).style(style)
        })
        .collect();
    
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::LightBlue));
    
    frame.render_widget(list, sidebar_chunks[1]);
    
    // Content area
    let content = chunks[1];
    
    match app.current_section {
        Section::System => render_system(frame, content, &app.settings),
        Section::Keybindings => render_keybindings(frame, content),
        Section::Theming => render_theming(frame, content),
        Section::Windows => render_windows(frame, content),
        Section::Workspaces => render_workspaces(frame, content),
        Section::Layout => render_layout(frame, content),
        Section::Bar => render_bar(frame, content),
        Section::Autostart => render_autostart(frame, content),
        Section::Picom => render_picom(frame, content),
        Section::Wallpaper => render_wallpaper(frame, content),
        Section::Profiles => render_profiles(frame, content),
    }
}

fn render_system(frame: &mut Frame, area: Rect, settings: &Settings) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(20), // ASCII art
            Constraint::Min(1),     // Info
            Constraint::Length(3), // Buttons
        ])
        .split(area);
    
    // ASCII art header
    let ascii = r#"
███████╗ █████╗ ████████╗███████╗██╗     ██╗███╗   ██╗███████╗
██╔════╝██╔══██╗╚══██╔══╝██╔════╝██║     ██║████╗  ██║██╔════╝
█████╗  ███████║   ██║   █████╗  ██║     ██║██╔██╗ ██║█████╗  
██╔══╝  ██╔══██║   ██║   ██╔══╝  ██║     ██║██║╚██╗██║██╔══╝  
██║     ██║  ██║   ██║   ███████╗███████╗██║██║ ╚████║███████╗
╚═╝     ╚═╝  ╚═╝   ╚═╝   ╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝╚══════╝
"#.trim();
    
    let ascii_widget = Paragraph::new(ascii)
        .style(Style::default().fg(Color::LightBlue))
        .alignment(Alignment::Center);
    frame.render_widget(ascii_widget, chunks[0]);
    
    // System info
    let info = format!(
        r#"OS: {}
Kernel: {}
CPU: {}
RAM: {}GB used / {}GB total
Disk: {}GB used / {}GB total"#,
        settings.os,
        settings.kernel,
        settings.cpu,
        settings.ram_used / 1024,
        settings.ram_total / 1024,
        settings.disk_used,
        settings.disk_total,
    );
    
    let info_widget = Paragraph::new(info)
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("System Information").borders(Borders::ALL));
    frame.render_widget(info_widget, chunks[1]);
    
    // Buttons
    let buttons = Paragraph::new("[Apply] [Reset] [Save] [Profiles]")
        .style(Style::default().fg(Color::LightBlue))
        .alignment(Alignment::Center);
    frame.render_widget(buttons, chunks[2]);
}

fn render_keybindings(frame: &mut Frame, area: Rect) {
    let widget = Paragraph::new("Keybindings Editor - Press Enter to modify")
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Keybindings").borders(Borders::ALL));
    frame.render_widget(widget, area);
}

fn render_theming(frame: &mut Frame, area: Rect) {
    let widget = Paragraph::new("Colors, transparency, font control")
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Theming").borders(Borders::ALL));
    frame.render_widget(widget, area);
}

fn render_windows(frame: &mut Frame, area: Rect) {
    let widget = Paragraph::new("Window rules, float settings, opacity")
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Windows").borders(Borders::ALL));
    frame.render_widget(widget, area);
}

fn render_workspaces(frame: &mut Frame, area: Rect) {
    let widget = Paragraph::new("Workspace count, names, monitors")
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Workspaces").borders(Borders::ALL));
    frame.render_widget(widget, area);
}

fn render_layout(frame: &mut Frame, area: Rect) {
    let widget = Paragraph::new("Gaps, borders, padding settings")
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Layout").borders(Borders::ALL));
    frame.render_widget(widget, area);
}

fn render_bar(frame: &mut Frame, area: Rect) {
    let widget = Paragraph::new("Bar modules, order, visibility")
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Bar").borders(Borders::ALL));
    frame.render_widget(widget, area);
}

fn render_autostart(frame: &mut Frame, area: Rect) {
    let widget = Paragraph::new("Startup applications")
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Autostart").borders(Borders::ALL));
    frame.render_widget(widget, area);
}

fn render_picom(frame: &mut Frame, area: Rect) {
    let widget = Paragraph::new("Picom backend, blur, animations")
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Picom").borders(Borders::ALL));
    frame.render_widget(widget, area);
}

fn render_wallpaper(frame: &mut Frame, area: Rect) {
    let widget = Paragraph::new("Wallpaper, color generators")
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Wallpaper").borders(Borders::ALL));
    frame.render_widget(widget, area);
}

fn render_profiles(frame: &mut Frame, area: Rect) {
    let widget = Paragraph::new("Gaming, Productivity, Battery, Custom profiles")
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Profiles").borders(Borders::ALL));
    frame.render_widget(widget, area);
}

fn main() {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .init();
    
    log::info!("Starting blink-conf");
    
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}
