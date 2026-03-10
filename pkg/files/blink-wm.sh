#!/bin/sh
# BlinkWM startup script

# Set XDG directories
export XDG_CONFIG_HOME="$HOME/.config"
export XDG_DATA_HOME="$HOME/.local/share"

# Start blink-bar (status bar)
if command -v blink-bar >/dev/null 2>&1; then
    blink-bar &
fi

# Start picom (compositor) if available
if command -v picom >/dev/null 2>&1; then
    picom -b &
fi

# Start notification daemon
if command -v dunst >/dev/null 2>&1; then
    dunst &
fi

# Start system tray apps
if command -v nm-applet >/dev/null 2>&1; then
    nm-applet &
fi

if command -v blueman-applet >/dev/null 2>&1; then
    blueman-applet &
fi

# Execute window manager
exec blink-wm
