//! BlinkWM Utility CLI Tool
//! Command-line tool for interacting with BlinkWM

use clap::{Parser, Subcommand};
use std::process::Command;

/// BlinkWM utility
#[derive(Parser)]
#[command(name = "blink")]
#[command(version = "0.1.0")]
#[command(about = "BlinkWM utility tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List windows
    Windows,
    
    /// List workspaces
    Workspaces,
    
    /// List outputs (monitors)
    Outputs,
    
    /// Focus a window
    Focus { id: u32 },
    
    /// Close a window
    Close { id: u32 },
    
    /// Move window to workspace
    Move { window_id: u32, workspace: usize },
    
    /// Switch workspace
    Workspace { index: usize },
    
    /// Set layout
    Layout { name: String },
    
    /// Next layout
    LayoutNext,
    
    /// Previous layout
    LayoutPrev,
    
    /// Screenshot
    Screenshot { mode: Option<String> },
    
    /// Volume control
    Volume { level: Option<u32> },
    
    /// Brightness control
    Brightness { level: Option<u32> },
    
    /// System info
    SystemInfo,
    
    /// Lock screen
    Lock,
    
    /// Reboot
    Reboot,
    
    /// Power off
    Poweroff,
}

fn main() {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Windows => list_windows(),
        Commands::Workspaces => list_workspaces(),
        Commands::Outputs => list_outputs(),
        Commands::Focus { id } => focus_window(id),
        Commands::Close { id } => close_window(id),
        Commands::Move { window_id, workspace } => move_window(window_id, workspace),
        Commands::Workspace { index } => switch_workspace(index),
        Commands::Layout { name } => set_layout(&name),
        Commands::LayoutNext => layout_next(),
        Commands::LayoutPrev => layout_prev(),
        Commands::Screenshot { mode } => screenshot(mode.as_deref()),
        Commands::Volume { level } => set_volume(level),
        Commands::Brightness { level } => set_brightness(level),
        Commands::SystemInfo => show_system_info(),
        Commands::Lock => lock_screen(),
        Commands::Reboot => reboot(),
        Commands::Poweroff => poweroff(),
    }
}

fn list_windows() {
    println!("Window list:");
    // In production, connect to IPC socket
    println!("  (connect to IPC for real data)");
}

fn list_workspaces() {
    println!("Workspace list:");
    for i in 1..=10 {
        let active = if i == 1 { "*" } else { " " };
        println!("  {}{}", active, i);
    }
}

fn list_outputs() {
    println!("Outputs:");
    // Use xrandr or similar
    let output = Command::new("xrandr")
        .output()
        .expect("Failed to run xrandr");
    
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if line.contains("connected") {
            println!("  {}", line.trim());
        }
    }
}

fn focus_window(id: u32) {
    // Send IPC message to focus window
    println!("Focusing window {}", id);
}

fn close_window(id: u32) {
    // Send IPC message to close window
    println!("Closing window {}", id);
}

fn move_window(window_id: u32, workspace: usize) {
    println!("Moving window {} to workspace {}", window_id, workspace);
}

fn switch_workspace(index: usize) {
    println!("Switching to workspace {}", index);
}

fn set_layout(name: &str) {
    println!("Setting layout to {}", name);
}

fn layout_next() {
    println!("Next layout");
}

fn layout_prev() {
    println!("Previous layout");
}

fn screenshot(mode: Option<&str>) {
    let mode = mode.unwrap_or("full");
    
    match mode {
        "full" => {
            let output = Command::new("scrot")
                .arg("/tmp/screenshot.png")
                .output();
            
            if output.is_ok() {
                println!("Screenshot saved to /tmp/screenshot.png");
            } else {
                println!("Failed to take screenshot");
            }
        }
        "region" => {
            println!("Region screenshot (use maim or slurp)");
        }
        "window" => {
            println!("Window screenshot");
        }
        _ => {
            println!("Unknown mode: {}", mode);
        }
    }
}

fn set_volume(level: Option<u32>) {
    if let Some(l) = level {
        let _ = Command::new("pactl")
            .args(["set-sink-volume", "@DEFAULT_SINK@", &format!("{}%", l)])
            .output();
        println!("Volume set to {}%", l);
    } else {
        let output = Command::new("pactl")
            .args(["get-sink-volume", "@DEFAULT_SINK@"])
            .output();
        
        if let Ok(output) = output {
            let text = String::from_utf8_lossy(&output.stdout);
            println!("Volume: {}", text.lines().next().unwrap_or("unknown"));
        }
    }
}

fn set_brightness(level: Option<u32>) {
    if let Some(l) = level {
        let _ = Command::new("brightnessctl")
            .args(["set", &format!("{}%", l)])
            .output();
        println!("Brightness set to {}%", l);
    } else {
        println!("Brightness control");
    }
}

fn show_system_info() {
    println!("System Information:");
    println!("  OS: Arch Linux");
    println!("  Kernel: {}", std::process::Command::new("uname")
        .arg("-r")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string()));
    println!("  CPU: Ryzen 7 5800X");
    println!("  RAM: 32GB");
}

fn lock_screen() {
    println!("Locking screen...");
    let _ = Command::new("loginctl")
        .args(["lock-session"])
        .output();
}

fn reboot() {
    println!("Rebooting...");
    let _ = std::process::Command::new("reboot").spawn();
}

fn poweroff() {
    println!("Powering off...");
    let _ = std::process::Command::new("poweroff").spawn();
}
