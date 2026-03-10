# BlinkWM - Complete Project Plan

---

## 1. Project Overview

**BlinkWM** - A minimal, fast, macOS-inspired hybrid tiling window manager for X11 written in Rust.

### Design Goals
- Blazing fast (<2s startup, <50MB RAM)
- Potato PC friendly
- Minimal + Fresh + Modern macOS Coder design
- VS Code Dark theme default

### Version Requirements
- Use **latest stable** versions of all dependencies
- Rust: stable channel (run `rustup update stable`)
- All crates: latest version from crates.io
- All system packages: latest from official repos/AUR

---

## 2. TUI Layouts

### blink-launch (App Launcher)
- **Style**: Spotlight popup
- **Size**: 600x400px (centered)
- **Position**: Center of screen

```
┌─────────────────────────────────────────────────────────────┐
│  > Search apps...                               ⌘  ✕       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Type to search...                                          │
│                                                             │
└─────────────────────────────────────────────────────────────┘

[After typing]:
┌─────────────────────────────────────────────────────────────┐
│  > term                                          ⌘  ✕       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Alacritty                     󰜝 Run in terminal          │
│  Kitty                         󰀮 Add to favorites         │
│  Terminal                      󌋢 App info                 │
│  URxvt                                                 │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### blink-conf (Config Tool)
- **Style**: Compact TUI
- **Size**: 900x650px (centered)
- **Position**: Center of screen

```
┌──────────────────────────────────────────────────────────────┐
│  󰘘 blink-conf                               > Search    ⌘   │
├─────────────────┬────────────────────────────────────────────┤
│                 │                                            │
│  󰒖 System      │    ██╗    ██╗ █████╗ ██████╗ ██╗        │
│                 │    ██║    ██║██╔══██╗██╔══██╗██║        │
│  󌌵 Keybindings │    ██║ █╗ ██║███████║██████╔╝██║        │
│                 │    ██║███╗██║██╔══██║██╔══██╗██║        │
│  󰈞 Theming     │    ╚███╔███╔╝██║  ██║██║  ██║███████╗   │
│                 │     ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝   │
│  󰘚 Windows     │                                            │
│                 │  OS: Arch Linux x86_64                     │
│  󰑹 Workspaces  │  Kernel: 6.x.x-arch1                      │
│                 │  CPU: Ryzen 7 5800X                        │
│  󰡒 Layout     │  RAM: 32GB used / 32GB total             │
│                 │  Disk: 256GB NVMe                          │
│  󰡜 Bar        │                                            │
│                 │  ┌──────────────────────────────────────┐    │
│  󰐞 Autostart   │  │ [Apply] [Reset] [Save] [Profiles] │    │
│                 │  └──────────────────────────────────────┘    │
│  󰾆 Picom       │                                            │
│                 │                                            │
│  󰍜 Wallpaper   │                                            │
│                 │                                            │
│  󰡱 Profiles    │                                            │
│                 │                                            │
└─────────────────┴────────────────────────────────────────────┘
```

### blink Utility Menu
- **Style**: Spotlight popup
- **Size**: 600x450px (centered)
- **Position**: Center of screen

```
┌─────────────────────────────────────────────────────────────┐
│  咪 Volume: ████████████░░░░░░░  65%    󰼡 Brightness   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   󰘚       󰤨       󰺀       󰡭                           │
│   Window   Output    WS       Layout                        │
│                                                             │
│   󰑔       󰋡       󰾔      󰒓                           │
│   Shot     Clip     Media    System                         │
│                                                             │
│   󰙅       󰋡       󰒓      󰒓                           │
│   Service  Proc     Net      Session                        │
│                                                             │
│   󰆞       󰔞       󰡃                                     │
│   Pkg      Notif    Config                                │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### blink-pkg (Package Installer)
- **Style**: Compact popup
- **Size**: 700x500px (centered)
- **Position**: Center of screen

```
┌─────────────────────────────────────────────────────────────┐
│  > Search packages...                            ⌘    ✕    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  □ alacritty   terminal emulator            [core]   1.2MB │
│  □ firefox     web browser                 [extra] 150MB  │
│  □ vscode      code editor                 [AUR]   200MB  │
│  □ discord     chat app                    [AUR]   120MB  │
│  ☑ neovim      text editor                 [extra]  15MB   │
│                                                             │
│  ─────────────────────────────────────────────────────────  │
│                                                             │
│  Install   Remove   Update   Upgrade   Sync      Info      │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Minimal Status Bar (Default)
- **Position**: Top
- **Height**: 24px

```
┌─────────────────────────────────────────────────────────────────────┐
│ [1] [2] [3] │ Firefox - BlinkWM │ 󰘚 12% │ 󰈐 45% │ 󰰯 Wed Mar 11 10:30 │
└─────────────────────────────────────────────────────────────────────┘
  ↑                              ↑                        ↑                   ↑
  Workspaces                    Center title             System stats        Clock
```

### Full Status Bar (Optional)
- **Position**: Top
- **Height**: 24px

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ [1] [2] [3] │ BSP │ 󰒖 Firefox │ 󰤨 WiFi │ 󰕫 85% │ 咪 65% │ 󰼡 80% │ 󰈔 55°C │ 󰽢 │ 󰕫 │ 󰰯 Wed Mar 11 10:30 │ ✉ │
└─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
  ↑        ↑      ↑       ↑         ↑        ↑       ↑        ↑      ↑    ↑    ↑         ↑          ↑
Workspaces Layout Title   WiFi    Battery   Volume  Brightness Temp   Media Notif Clock      Tray
```

---

## 3. Architecture

```
BlinkWM/
├── blink-wm/           # Core window manager (Rust)
├── blink-launch/      # App launcher TUI
├── blink-conf/        # Config TUI
├── blink-bar/         # Status bar
├── blink/             # Utility CLI tool
├── blink-pkg/         # Package installer (nala-like)
└── blink-ipc/         # Shared IPC library
```

### Tech Stack
| Component | Technology |
|-----------|------------|
| WM Core | Rust + x11rb |
| Keyboard | xkbcommon |
| Config | Lua (rlua) |
| TUI Framework | Ratatui + Crossterm |
| IPC | Unix socket (i3-compatible) |

---

## 4. Core Window Manager (blink-wm)

### Features
| Feature | Description |
|---------|-------------|
| **Hybrid Tiling** | Tiling + Floating modes |
| **Layouts** | BSP, Grid, Main+Stack, Monocle, Spiral |
| **Workspaces** | Dynamic, multi-monitor support |
| **Scratchpad** | Hide/show windows with shortcut |
| **Smart Borders** | Hide when single window |
| **Smart Gaps** | No gaps when single window |
| **Marked Windows** | Mark for batch operations |
| **Window Rules** | Per-app float/size/position/opacity |
| **Per-workspace Layout** | Different layout per workspace |
| **Auto-floating** | Auto-float dialogs |
| **Global Fullscreen** | Fullscreen across monitors |
| **Focus Mode** | Click to focus + sloppy focus |

### Standards
- EWMH/ICCCM compliance
- X11 session management

---

## 5. Status Bar (blink-bar)

### Default (Minimal)
| Position | Content |
|----------|---------|
| Left | Workspaces (occupied only) |
| Center | Focused window title |
| Right | CPU % \| RAM % \| Date/Time |

### Full Bar (Optional)
- Workspaces, Layout indicator, App launcher
- WiFi, Battery, Volume, Brightness
- CPU temp, Media player, Notifications
- System tray, Clock

### Features
- Built-in + i3bar compatible
- Top position (macOS style)
- 24px height
- Drag & drop module reordering
- Enable/disable modules
- Theme adaptive colors

---

## 6. App Launcher (blink-launch)

### Design
- Spotlight-style popup (600x400px, centered)
- No icons - text only
- `>` prefix in search bar
- Lazy loading - no preloading
- Blazing fast fuzzy search

### Features
- Search apps on type
- Recent apps at top
- Right-click context menu:
  - Run
  - Run in Terminal
  - Run as Root
  - Add to Favorites

---

## 7. Config Tool (blink-conf)

### Interface
- Compact TUI (900x650px, centered)
- Sidebar navigation
- Default page: System Info with ASCII logo

### Sections
| Section | Features |
|---------|----------|
| **System** | SysInfo (default page) |
| **Keybindings** | Visual editor, key recorder |
| **Theming** | Colors, transparency, font control |
| **Windows** | Window rules |
| **Workspaces** | Count, names, monitors |
| **Layout** | Gaps, borders, padding |
| **Bar** | Modules, order, visibility |
| **Autostart** | Startup apps |
| **Picom** | Backend, blur, animations |
| **Wallpaper** | Set wallpaper, generators |
| **Profiles** | Gaming, Productivity, Battery, Custom |

### Colorscheme Format (Key=Value Tags)
```ini
Borders=#3c3c3c
BordersActive=#007ACC
Background=#1e1e1e
Surface=#252526
Text=#cccccc
Accent=#007ACC
```

---

## 8. Utility Tool (blink)

### Compact Menu (600x450px)
- Volume slider + brightness slider at top
- 3-column icon grid
- Opens config TUI via Settings

### Commands
| Category | Commands |
|----------|----------|
| **Window** | list, move, resize, close, focus, raise, lower |
| **Output** | list, enable, disable, primary, mode |
| **Workspace** | switch, move, create, delete, rename |
| **Layout** | list, set, next, prev |
| **Screenshot** | full, region, window, clipboard |
| **Clipboard** | copy, paste, history, clear |
| **Media** | vol, bright, play, pause, next, prev |
| **System** | cpu, mem, battery, disk |
| **Services** | list, start, stop, restart |
| **Process** | list, kill, top |
| **Packages** | pacman + AUR (search, install, remove, update) |
| **VPN** | WireGuard, OpenVPN, NM profiles |
| **DNS** | Presets (Cloudflare, Google, Quad9, OpenDNS) + custom |
| **Modem** | List, enable/disable, signal |
| **Network** | WiFi list, connect, disconnect |
| **Notifications** | send, list, clear |
| **Session** | lock, sleep, reboot, poweroff, logout |

---

## 9. Package Installer (blink-pkg)

### Design
- Custom Nala-like TUI (clean, beautiful UI)
- Compact popup (600x450px, centered)
- `>` prefix in search bar

### Features
- Fuzzy search packages
- Multi-select install with checkboxes
- Install/Remove/Update with progress bars
- Show package details (deps, size, repo)
- Clean color output

### UI Layout
```
┌─────────────────────────────────────────────────────────────┐
│  > Search packages...                            ⌘    ✕    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  □ alacritty  terminal emulator              [core]   1.2MB │
│  □ firefox    web browser                  [extra]  150MB  │
│  □ vscode    code editor                   [AUR]   200MB   │
│  □ discord   chat app                      [AUR]   120MB   │
│                                                             │
│  [Install] [Remove] [Update] [Info]                        │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Backend
- Uses pacman for official repos
- Uses yay/paru for AUR

---

## 10. Performance

| Metric | Target |
|--------|--------|
| Startup | <2 seconds |
| Memory | <50MB (WM core) |
| CPU Usage | Near zero idle |

### Optimizations
- Event-driven UI (no polling)
- Throttled updates (CPU/RAM: 5-10s, Clock: 1min)
- Lazy loading (app launcher)
- Async Rust (Tokio)
- Minimal dependencies (pure Rust)
- Smart caching

---

## 11. Dependencies

### Required (Arch Linux)
- **Rust** (official: `rust`)
- **x11rb** (crate - not a system package)
- **xkbcommon** (official: `libxkbcommon`)
- **rlua** (crate - not a system package)
- **ratatui** + **crossterm** (crates - not system packages)
- **JetBrains Mono** (official: `ttf-jetbrains-mono`)
- **Nerd Fonts** (AUR: `ttf-jetbrains-mono-nerd` or `nerd-fonts-complete`)

### Optional (Arch Linux)
- **hellwal** (AUR)
- **wallust** (AUR)
- **pywal** (official: `python-pywal`)
- **picom-git** (AUR - recommended for blur/animation)

---

## 12. Todo List

### Phase 1: Core Window Manager
- [ ] Set up project structure (Cargo workspace with all crates)
- [ ] Implement X11 connection and event loop in blink-wm core
- [ ] Implement window management (detect, reparent, manage windows)
- [ ] Implement tiling layouts (BSP, Grid, Main+Stack, Monocle, Spiral)
- [ ] Implement workspace management (dynamic, multi-monitor)
- [ ] Implement keybinding system with xkbcommon
- [ ] Implement floating mode and window rules
- [ ] Implement scratchpad feature
- [ ] Implement smart borders and smart gaps
- [ ] Implement EWMH/ICCCM compliance
- [ ] Implement IPC socket server (i3-compatible)
- [ ] Implement Lua config parsing
- [ ] Implement colorscheme system with Key=Value format

### Phase 2: Status Bar
- [ ] Build blink-bar status bar (minimal default)
- [ ] Add bar modules (CPU, RAM, workspaces, title)
- [ ] Add full bar modules (optional)

### Phase 3: App Launcher
- [ ] Build blink-launch app launcher TUI
- [ ] Implement fuzzy search with lazy loading
- [ ] Add context menu for launcher

### Phase 4: Config Tool
- [ ] Build blink-conf config TUI
- [ ] Implement theming section with font control
- [ ] Implement profile system (Gaming, Productivity, Battery)

### Phase 5: Utility Tool + Package Installer
- [ ] Build blink utility CLI tool
- [ ] Implement window operations (list, move, resize, close)
- [ ] Implement screenshot functionality
- [ ] Implement clipboard manager
- [ ] Implement media controls (volume, brightness, player)
- [ ] Implement system info (CPU, RAM, battery)
- [ ] Build blink-pkg package installer TUI
- [ ] Implement fuzzy search for packages
- [ ] Implement multi-select install/remove
- [ ] Implement progress display with colors
- [ ] Implement package details (deps, size, repo)
- [ ] Integrate pacman + AUR backend
- [ ] Implement network controls (WiFi, VPN, DNS, Modem)
- [ ] Implement session controls (lock, sleep, reboot, poweroff)

### Phase 6: Utility Menu
- [ ] Build compact utility menu TUI
- [ ] Add volume/brightness sliders to utility menu

### Phase 7: Integrations
- [ ] Integrate picom configuration in blink-conf
- [ ] Add wallpaper/color generator integration (hellwal, wallust, pywal)
- [ ] Implement X session management
- [ ] Add display manager integration

### Phase 8: Optimization & Polish
- [ ] Performance optimization (event-driven, throttled updates)
- [ ] Create default config files and colorschemes
- [ ] Test and polish all components

---

## 13. Arch Linux Packaging

### PKGBUILD Structure
```
blink-wm/
├── PKGBUILD
├── .SRCINFO
└── files/
    ├── blink-wm.service
    ├── blink.desktop
    └── blink-wm.sh
```

### Package Contents
```
usr/bin/blink-wm         # Main executable
usr/bin/blink            # Utility CLI
usr/bin/blink-launch     # App launcher
usr/bin/blink-conf       # Config tool
usr/bin/blink-bar        # Status bar
usr/bin/blink-pkg        # Package installer
usr/lib/systemd/system/blink-wm.service  # Systemd service
usr/share/xsessions/blink.desktop       # LightDM/SDDM
usr/share/blink-wm/      # Config directory
```

### Dependencies (Arch Linux)
- rust
- libxkbcommon
- ttf-jetbrains-mono
- ttf-jetbrains-mono-nerd (AUR)

### Optional
- picom-git (AUR)
- hellwal (AUR)
- wallust (AUR)
- python-pywal (official)
