//! BlinkWM App Launcher

use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, List, ListItem},
    layout::{Layout, Constraint, Direction, Alignment},
    prelude::Rect,
    Frame, Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;
use std::path::PathBuf;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

struct App {
    query: String,
    results: Vec<AppEntry>,
    selected_index: usize,
    quit: bool,
}

#[derive(Debug, Clone)]
struct AppEntry {
    name: String,
    exec: String,
    terminal: bool,
    comment: String,
}

impl Default for App {
    fn default() -> Self {
        let apps = load_apps();
        Self { query: String::new(), results: apps, selected_index: 0, quit: false }
    }
}

impl App {
    fn filter(&mut self) {
        if self.query.is_empty() { self.results = load_apps(); return; }
        let matcher = SkimMatcherV2::default();
        let mut filtered: Vec<_> = load_apps().into_iter().filter_map(|app| {
            let score = matcher.fuzzy_match(&app.name, &self.query);
            score.map(|s| (s, app))
        }).collect();
        filtered.sort_by(|a, b| b.0.cmp(&a.0));
        self.results = filtered.into_iter().map(|(_, app)| app).collect();
        self.selected_index = 0;
    }
    fn move_up(&mut self) { if self.selected_index > 0 { self.selected_index -= 1; } }
    fn move_down(&mut self) { if self.selected_index < self.results.len().saturating_sub(1) { self.selected_index += 1; } }
    fn execute(&self) {
        if let Some(app) = self.results.get(self.selected_index) {
            let _ = std::process::Command::new("sh").args(["-c", &format!("{} &", app.exec)]).spawn();
        }
    }
}

fn load_apps() -> Vec<AppEntry> {
    let mut apps = Vec::new();
    let dirs = vec![PathBuf::from("/usr/share/applications"), PathBuf::from("/usr/local/share/applications")];
    for dir in dirs {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |e| e == "desktop") {
                    if let Ok(app) = parse_desktop_file(&path) { apps.push(app); }
                }
            }
        }
    }
    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps.dedup_by(|a, b| a.name == b.name);
    apps
}

fn parse_desktop_file(path: &PathBuf) -> Result<AppEntry, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let mut name = String::new(); let mut exec = String::new(); let mut terminal = false; let mut comment = String::new(); let mut no_display = false;
    for line in content.lines() {
        if line.starts_with("NoDisplay=true") || line.starts_with("Hidden=true") { no_display = true; break; }
        if let Some(v) = line.strip_prefix("Name=") { name = v.to_string(); }
        else if let Some(v) = line.strip_prefix("Exec=") { exec = v.split_whitespace().next().unwrap_or("").to_string(); }
        else if let Some(v) = line.strip_prefix("Terminal=") { terminal = v == "true"; }
        else if let Some(v) = line.strip_prefix("Comment=") { comment = v.to_string(); }
    }
    if no_display || name.is_empty() || exec.is_empty() { return Err("Invalid".into()); }
    Ok(AppEntry { name, exec, terminal, comment })
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
                    KeyCode::Char(c) => { app.query.push(c); app.filter(); }
                    KeyCode::Backspace => { app.query.pop(); app.filter(); }
                    KeyCode::Up => app.move_up(),
                    KeyCode::Down => app.move_down(),
                    KeyCode::Enter => { app.execute(); break; }
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
    let width = 60u16; let height = 20u16;
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;
    let popup = Rect::new(x, y, width, height);
    let chunks = Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(3), Constraint::Min(1)]).split(popup);
    let search = Paragraph::new(format!(" > {} ", app.query)).style(Style::default().fg(Color::White)).block(Block::default().borders(Borders::ALL).title("Search apps...")).alignment(Alignment::Left);
    frame.render_widget(search, chunks[0]);
    let item_style = Style::default().fg(Color::White);
    let selected_style = Style::default().fg(Color::LightBlue).bg(Color::DarkGray);
    let items: Vec<ListItem> = app.results.iter().enumerate().map(|(i, entry)| {
        let style = if i == app.selected_index { selected_style } else { item_style };
        ListItem::new(format!("{} ({})", entry.name, entry.comment)).style(style)
    }).collect();
    let list = List::new(items).block(Block::default().borders(Borders::ALL)).highlight_style(selected_style);
    frame.render_widget(list, chunks[1]);
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    log::info!("Starting blink-launch");
    if let Err(e) = run() { eprintln!("Error: {}", e); }
}
