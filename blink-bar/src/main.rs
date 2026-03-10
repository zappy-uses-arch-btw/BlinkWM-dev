//! BlinkWM Status Bar

use ratatui::{
    style::{Color, Style},
    widgets::{Block, Paragraph},
    layout::{Layout, Constraint, Direction},
    Frame, Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;
use std::process::Command;

struct App {
    workspaces: Vec<WorkspaceState>,
    focused_title: String,
    cpu_percent: f32,
    ram_used: u64,
    ram_total: u64,
    quit: bool,
}

#[derive(Clone)]
struct WorkspaceState {
    name: String,
    index: usize,
    has_windows: bool,
    is_active: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            workspaces: (1..=10).map(|i| WorkspaceState {
                name: format!("{}", i),
                index: i - 1,
                has_windows: false,
                is_active: i == 1,
            }).collect(),
            focused_title: "BlinkWM".to_string(),
            cpu_percent: 0.0,
            ram_used: 0,
            ram_total: 0,
            quit: false,
        }
    }
}

impl App {
    fn update_stats(&mut self) {
        if let Ok(output) = Command::new("sh")
            .args(["-c", "cat /proc/stat | head -1 | awk '{print ($2+$4+$6)/($2+$4+$5+$6)*100}'"])
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout);
            self.cpu_percent = text.trim().parse().unwrap_or(0.0);
        }
        
        if let Ok(output) = Command::new("sh")
            .args(["-c", "cat /proc/meminfo | grep 'MemAvailable' | awk '{print $2}'"])
            .output()
        {
            let available: u64 = String::from_utf8_lossy(&output.stdout).trim().parse().unwrap_or(0);
            
            if let Ok(output) = Command::new("sh")
                .args(["-c", "cat /proc/meminfo | grep 'MemTotal' | awk '{print $2}'"])
                .output()
            {
                let total: u64 = String::from_utf8_lossy(&output.stdout).trim().parse().unwrap_or(0);
                self.ram_total = total;
                self.ram_used = total.saturating_sub(available);
            }
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let mut app = App::default();
    app.update_stats();
    
    loop {
        terminal.draw(|f| ui(f, &app))?;
        
        if event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc {
                    break;
                }
            }
        }
        
        app.update_stats();
        
        if app.quit { break; }
    }
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    
    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    let area = frame.size();
    
    let chunks: Vec<ratatui::prelude::Rect> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(10), Constraint::Length(30)])
        .split(area)
        .to_vec();
    
    let inactive_style = Style::default().fg(Color::White).bg(Color::DarkGray);
    
    let ws_chunk = chunks[0];
    let workspace_widget = Paragraph::new(
        app.workspaces.iter().map(|ws| {
            if ws.is_active { format!("[{}]", ws.name) } else { format!(" {} ", ws.name) }
        }).collect::<Vec<_>>().join("")
    )
    .style(inactive_style)
    .block(Block::default());
    frame.render_widget(workspace_widget, ws_chunk);
    
    let title_chunk = chunks[1];
    let title_widget = Paragraph::new(app.focused_title.as_str())
        .style(inactive_style)
        .block(Block::default())
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(title_widget, title_chunk);
    
    let right_chunk = chunks[2];
    let ram_percent = if app.ram_total > 0 {
        (app.ram_used as f64 / app.ram_total as f64 * 100.0) as f32
    } else {
        0.0
    };
    let right_text = format!(" CPU {:.0}%  RAM {:.0}%  {} ", app.cpu_percent, ram_percent, get_clock_string());
    let right_widget = Paragraph::new(right_text)
        .style(inactive_style)
        .block(Block::default())
        .alignment(ratatui::layout::Alignment::Right);
    frame.render_widget(right_widget, right_chunk);
}

fn get_clock_string() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let day_secs = now % 86400;
    let hour = day_secs / 3600;
    let minute = (day_secs % 3600) / 60;
    
    let days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let day = days[((now / 86400) % 7) as usize];
    
    format!("{} {:02}:{:02}", day, hour, minute)
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    log::info!("Starting blink-bar");
    
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}
