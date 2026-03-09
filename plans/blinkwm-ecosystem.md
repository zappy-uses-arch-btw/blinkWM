# BlinkWM Ecosystem Plan

## Overview
| | |
|---|---|
| **Type** | X11 Window Manager + Desktop Environment |
| **Language** | Rust |
| **Design** | macOS-inspired, dark, minimal, sleek |
| **Organization** | Monorepo |

---

## Design System

### Colors (Dark Default)
```
Background:   #1E1E1E
Background2:  #2D2D2D
Accent:       #007AFF
Text:         #FFFFFF
Text muted:   #8E8E93
Border:       #3D3D3D
Success:      #30D158
Warning:      #FF9F0A
Error:        #FF453A
```

### Key Principles
- Sleek, modern, minimal - no bloat
- Fast (Rust-powered)
- Keyboard-first
- Pure Rust where possible

---

## Components (7 Crates)

| Crate | Purpose |
|-------|---------|
| **blinkwm** | Core X11 WM |
| **blinkwm-bar** | Status bar (modular) |
| **blinkwm-dmenu** | App launcher + utilities |
| **blinkwm-tray** | System tray |
| **blinkwm-config** | TUI config tool |
| **blinkwm-session** | Session manager |
| **blinkwm-picom** | Compositor config |

---

## Core WM Features

### Gaps & Borders
- Smart gaps (hide when single window)
- Smart borders (hide when single window)
- Border colors: focused/unfocused/urgent

### Window Management
- Window rules (by class/instance/title)
- Scratchpad support
- Floating/tiling modes

### Layouts
- Vertical/Horizontal split
- Stacking
- Fullscreen

### Keybinds (Default)
```
MOD+J/K/H/L     Focus        | MOD+1-9       Workspace
MOD+SHIFT+...   Move        | MOD+TAB       Next ws
MOD+ALT+...     Swap        | MOD+ENTER     Terminal
MOD+F           Float       | MOD+D         Launcher
MOD+SPACE       Layout      | MOD+Q         Close
MOD+R           Resize      | MOD+SHIFT+R   Reload
```

---

## App Launcher (blinkwm-dmenu)

### Modes
| Mode | Trigger | Description |
|------|---------|-------------|
| Grid | MOD+D | Icon grid + search |
| Run | MOD+ALT+D | Command runner |
| Window | MOD+TAB | Switch windows |

### Features
- Fuzzy search
- Recent apps
- Categories
- Custom shortcuts
- **Search bar at bottom** with `>` prefix (configurable)

### Prefix Commands
```
> cmd        Run command
>= 2+2       Calculator (inline)
>? man       Man pages
c            Color picker
```

---

## Status Bar (blinkwm-bar)

### Default Modules
- Workspaces (occupied only)
- Title (focused window)
- CPU / RAM
- Date/Time
- Layout indicator

### Configurable
- Enable/disable modules
- Reorder via config
- External bar support (polybar, etc.)

---

## Config Tool (blinkwm-config)

### Sections (8)
1. Dashboard
2. Keybindings
3. Workspaces
4. Window Rules
5. Theming
6. Compositor (picom)
7. Profiles
8. Startup

### Theming
- 10 built-in themes
- Wallpaper generators: pywal16, wallust, hellwal (auto-detect)
- Custom themes
- Fonts: UI, Bar, Terminal, Icon

### Color Editing (Labeled)
```
Background    = #1E1E1E ██ [Edit]
Text Primary = #FFFFFF   ██ [Edit]
Accent       = #007AFF   ██ [Edit]
Border Focus = #007AFF   ██ [Edit]
```

### Profiles
- Default, Work, Gaming, Presentation, Night
- Quick switch: MOD+ALT+W/G/P/N

---

## Compositor (picom)

### Settings
- Rounded corners (default: 12px)
- Shadows, Fade, Blur
- Backend: glx / xrender / hybrid

### Blur (Auto-Detect)
- dual_kawase ✓ (recommended)
- kernel ✓
- box ✗

### App Rules
- Per-app opacity
- Blur exclusion
- Shadow exclusion

---

## Utilities (blinkwm-utils)

### Access: MOD+U

| Tool | Description |
|------|-------------|
| Calculator | `>= 2+2` inline |
| Process Manager | View/kill processes |
| Color Picker | Pick from screen |
| Keybindings | Search all binds |
| System Monitor | CPU/RAM/Disk/Temp |

### Network
| Feature | Support |
|---------|---------|
| WiFi | Toggle, manage |
| VPN | WireGuard, OpenVPN, etc |
| DNS | DoH, DoT, Custom |
| Modem | USB modem, SIM, APN |

---

## Package Structure
```
blinkwm/
├── Cargo.toml
├── packages/
│   ├── blinkwm/
│   ├── blinkwm-bar/
│   ├── blinkwm-dmenu/
│   ├── blinkwm-tray/
│   ├── blinkwm-config/
│   └── blinkwm-session/
├── themes/
├── configs/
└── PKGBUILD
```

---

## Quick Start
```bash
# Install deps
sudo pacman -S picom

# Run via xsession
exec blinkwm
```

---

## Key Differentiators

| Feature | Why Great |
|---------|-----------|
| Unified launcher | Everything from one place |
| Search bar at bottom | dmenu style, clean |
| All-in-one config | No editing config files |
| Profiles | Instant work/gaming modes |
| Wallpaper theming | pywal/wallust/hellwal |
| VPN/DNS/Modem | Full network control |
| Calculator inline | `>= 2+2` anywhere |
