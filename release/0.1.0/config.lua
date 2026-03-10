-- BlinkWM Configuration
-- Uses Key=Value format for simple settings

-- General
terminal = "alacritty"
file_manager = "thunar"
menu = "blink-launch"

-- Workspaces
workspace_count = 10

-- Layout
default_layout = "BSP"
border_width = 2
smart_gaps = true
smart_borders = true

-- Focus
sloppy_focus = true
focus_follows_mouse = false
float_dialogs = true

-- Keybindings are defined in the binary

-- Float classes (these apps will float by default)
float_classes = {
    "pavucontrol",
    "nm-connection-editor",
    "blueman-manager",
    "yad",
    "zenity",
}

-- Colorscheme
-- Uses Key=Value format
borders = "#3c3c3c"
borders_active = "#007ACC"
background = "#1e1e1e"
surface = "#252526"
text = "#cccccc"
text_dim = "#808080"
accent = "#007ACC"
urgent = "#FF5555"
bar_bg = "#1e1e1e"
bar_fg = "#cccccc"

-- Autostart applications
autostart = {
    "picom",
    "blink-bar",
    "nm-applet",
    "blueman-applet",
    "volumeicon",
}
