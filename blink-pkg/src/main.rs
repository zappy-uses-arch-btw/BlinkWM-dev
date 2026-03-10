//! BlinkWM Package Installer

use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, List, ListItem},
    layout::{Layout, Constraint, Direction, Alignment},
    Frame, Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;

struct App {
    query: String,
    results: Vec<PackageEntry>,
    selected: usize,
    quit: bool,
}

#[derive(Debug, Clone)]
struct PackageEntry {
    name: String,
    version: String,
    repo: String,
    description: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            query: String::new(),
            results: Vec::new(),
            selected: 0,
            quit: false,
        }
    }
}

impl App {
    fn search(&mut self) {
        if self.query.is_empty() {
            self.results = Vec::new();
            return;
        }
        
        if let Ok(output) = std::process::Command::new("pacman")
            .args(["-Ss", &self.query])
            .output()
        {
            let mut packages = Vec::new();
            let mut current_name = String::new();
            let mut current_repo = String::new();
            let mut current_version = String::new();
            let mut current_description = String::new();
            
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                if line.starts_with(":: ") || line.starts_with("aur/") || line.contains('/') {
                    if !current_name.is_empty() {
                        packages.push(PackageEntry {
                            name: current_name.clone(),
                            version: current_version.clone(),
                            repo: current_repo.clone(),
                            description: current_description.clone(),
                        });
                    }
                    
                    if let Some((repo, rest)) = line.split_once('/') {
                        current_repo = repo.to_string();
                        let parts: Vec<&str> = rest.split_whitespace().collect();
                        current_name = parts.first().unwrap_or(&"").to_string();
                        current_version = parts.get(1).unwrap_or(&"?").to_string();
                    }
                    current_description = String::new();
                } else if line.starts_with("    ") {
                    current_description = line.trim().to_string();
                }
            }
            
            if !current_name.is_empty() {
                packages.push(PackageEntry {
                    name: current_name,
                    version: current_version,
                    repo: current_repo,
                    description: current_description,
                });
            }
            
            self.results = packages;
        }
        
        self.selected = 0;
    }
    
    fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }
    
    fn move_down(&mut self) {
        if self.selected < self.results.len().saturating_sub(1) {
            self.selected += 1;
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
    
    loop {
        terminal.draw(|f| ui(f, &app))?;
        
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => { app.query.push(c); app.search(); }
                    KeyCode::Backspace => { app.query.pop(); app.search(); }
                    KeyCode::Up => app.move_up(),
                    KeyCode::Down => app.move_down(),
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
        
        if app.quit { break; }
    }
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    
    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    let area = frame.size();
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(3)])
        .split(area);
    
    let search = Paragraph::new(format!(" > {} ", app.query))
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Search packages..."))
        .alignment(Alignment::Left);
    frame.render_widget(search, chunks[0]);
    
    let items: Vec<ListItem> = app.results.iter().enumerate().map(|(i, pkg)| {
        let style = if i == app.selected { 
            Style::default().fg(Color::LightBlue) 
        } else { 
            Style::default().fg(Color::White) 
        };
        ListItem::new(format!(" {:20} {} [{}]", pkg.name, pkg.version, pkg.repo)).style(style)
    }).collect();
    
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::LightBlue).bg(Color::DarkGray));
    frame.render_widget(list, chunks[1]);
    
    let status = Paragraph::new("[Install] [Remove] [Update] [Info]")
        .style(Style::default().fg(Color::LightBlue))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    frame.render_widget(status, chunks[2]);
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    log::info!("Starting blink-pkg");
    
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}
