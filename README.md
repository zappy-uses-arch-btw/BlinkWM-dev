# BlinkWM

A minimal, fast, macOS-inspired hybrid tiling window manager for X11 written in Rust.

## Features

- **Hybrid Tiling**: Tiling + Floating modes
- **Layouts**: BSP, Grid, Main+Stack, Monocle, Spiral
- **10 Dynamic Workspaces**: Multi-monitor support
- **Spotlight-style App Launcher**: `blink-launch`
- **Config Tool**: TUI configuration editor - `blink-conf`
- **Status Bar**: Minimal bar with workspaces, title, CPU, RAM, clock - `blink-bar`
- **Package Installer**: Nala-like TUI for pacman - `blink-pkg`
- **Utility CLI**: Window/workspace control - `blink`

## Requirements

### Required
- Rust (stable)
- libxkbcommon

### Optional
- picom-git (for blur/animations)
- ttf-jetbrains-mono (default font)
- ttf-jetbrains-mono-nerd (nerd fonts)

## Building

```bash
# Clone and build
cargo build --release

# Or use the makefile
make
```

## Installation

### From Source

```bash
cargo install --path blink-wm
cargo install --path blink-launch
cargo install --path blink-conf
cargo install --path blink-bar
cargo install --path blink
cargo install --path blink-pkg
```

### Arch Linux (AUR)

```bash
# Using yay
yay -S blink-wm

# Or build manually
cd pkg
makepkg -si
```

## Usage

### Starting BlinkWM

#### Via display manager
Select "BlinkWM" from your login screen (LightDM, SDDM, etc.)

#### Via xinit
Add to `~/.xinitrc`:
```bash
exec blink-wm
```

Or with autostart apps:
```bash
exec /usr/share/blink-wm/blink-wm.sh
```

### Keybindings

| Key | Action |
|-----|--------|
| `Mod4 + Return` | Open terminal |
| `Mod4 + q` | Quit BlinkWM |
| `Mod4 + Shift + r` | Reload config |
| `Mod4 + j/k` | Focus next/prev window |
| `Mod4 + f` | Toggle fullscreen |
| `Mod4 + space` | Toggle float |
| `Mod4 + c` | Close window |
| `Mod4 + l/h` | Next/prev layout |
| `Mod4 + 1-0` | Switch to workspace 1-10 |
| `Mod4 + d` | App launcher |
| `Mod4 + Shift + c` | Config tool |

`Mod4` is typically the Super/Windows key.

### CLI Usage

```bash
# List windows
blink windows

# Switch workspace
blink workspace 1

# Screenshot
blink screenshot full
blink screenshot region

# Volume
blink volume 50
blink volume

# Lock screen
blink lock
```

## Configuration

Config file: `~/.config/blink-wm/config.lua`

### Example Config

```lua
terminal = "alacritty"
file_manager = "thunar"
default_layout = "BSP"
border_width = 2
smart_gaps = true
smart_borders = true
sloppy_focus = true

-- Float certain apps
float_classes = {
    "pavucontrol",
    "nm-connection-editor",
}

-- Colorscheme (Key=Value format)
borders = "#3c3c3c"
borders_active = "#007ACC"
background = "#1e1e1e"
text = "#cccccc"
accent = "#007ACC"
```

## Themes

Located in `/usr/share/blink-wm/colorschemes/`:
- vscode-dark (default)
- gruvbox-dark
- nord
- catppuccin
- onedark

## Components

| Binary | Description |
|--------|-------------|
| `blink-wm` | Window manager core |
| `blink-launch` | App launcher (Spotlight-style) |
| `blink-conf` | Configuration TUI |
| `blink-bar` | Status bar |
| `blink` | Utility CLI |
| `blink-pkg` | Package installer |

## Performance Targets

- Startup: <2 seconds
- Memory: <50MB (WM core)
- CPU: Near zero idle

## License

MIT
