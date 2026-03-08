# BlinkWM - Complete Implementation Plan

## Overview

A sleek, minimal, modern X11 window manager written in Rust with integrated TUIs for a complete desktop experience.

---

## 1. Project Structure

```
/home/bhishma/wm2/
├── blink/                    # Window Manager
├── tui/
│   ├── blink-ctl/           # Config TUI
│   ├── blink-bar/          # Status bar
│   ├── blink-launcher/     # App launcher (rofi)
│   ├── blink-keybinds/    # Keybind viewer
│   ├── blink-sys/        # Package manager, mirrors, users
│   ├── blink-system/     # System actions
│   ├── blink-controls/   # Network + Volume
│   └── blink-notifications/ # Notification history
├── libs/
│   ├── blink-theme/      # Shared theme colors
│   ├── blink-ipc/       # IPC client/server
│   └── blink-config/    # Config reader/writer (TOML)
├── config/               # Default configs
└── package/             # Arch Linux package scripts
```

---

## 2. Dependencies

### Core Dependencies
- rust (latest stable)
- penrose (WM framework)
- xorg-xinit
- libx11
- libxcb

### Optional Dependencies
- picom-git (compositor)
- dunst (notifications)
- rate-mirrors-bin (mirror selection)
- paru (AUR helper)

---

## 3. Config Format

All configs in TOML format, stored in `~/.config/blink/`

### Main Config: config.toml
```toml
# ~/.config/blink/config.toml

[general]
terminal = "alacritty"
launcher = "blink-launcher"
modkey = "Mod4"

[focus]
follow_mouse = true
click_to_focus = false
autoraise = false

[gaps]
inner = 5
outer = 0
smart_gaps = true
smart_border = true

[visual]
resize_indicator = true
tag_indicator = true
border_width = 2

[tray]
enable = false

[[tags]]
name = "一"
icon = "1"
layout = "tile"

[[tags]]
name = "二"
icon = "2"
layout = "tile"

# ... up to 9 tags

[[rules]]
class = "firefox"
tag = 2
float = false

[[autostart]]
command = "picom --experimental-backends"
command = "blink-bar"
command = "dunst"
```

### Keybinds: keybinds.toml
```toml
# ~/.config/blink/keybinds.toml

[[binding]]
key = "Mod+Return"
action = "spawn"
command = "alacritty"
category = "General"

[[binding]]
key = "Mod+d"
action = "spawn"
command = "blink-launcher"
category = "General"
```

### Theme: theme.toml
```toml
# ~/.config/blink/theme.toml

[colors]
background = "#1e1e2e"
foreground = "#cdd6f4"
accent = "#89b4fa"
success = "#a6e3a1"
warning = "#f9e2af"
error = "#f38ba8"
border = "#313244"

[colors.focused]
border = "#89b4fa"

[colors.unfocused]
border = "#4c566a"

[colors.urgent]
border = "#f38ba8"
```

---

## 4. IPC System

### Socket
- Dynamic socket: `/tmp/blink-{random}.sock`
- Server runs in WM
- All TUIs connect as clients

### Events (WM → TUIs)
| Event | Data |
|-------|------|
| tag_changed | {tag: 1} |
| focus_changed | {window: "firefox"} |
| layout_changed | {layout: "tile"} |
| client_list | {windows: [...]} |
| window_open | {id: 123} |
| window_close | {id: 123} |

### Commands (TUIs → WM)
| Command | Action |
|---------|--------|
| spawn | Launch app |
| spawn_root | Launch as root (pkexec) |
| reload | Reload config |
| quit | Quit WM |
| set_layout | Change layout |
| focus_tag | Switch tag |

---

## 5. Shared Libraries

### blink-theme
- Provides unified theme colors
- All TUIs use this for consistent look
- Reads from `~/.config/blink/theme.toml`

### blink-ipc
- IPC client for TUIs
- IPC server for WM
- Event subscription system

### blink-config
- Read/Write TOML configs
- Watch for file changes
- Hot reload support

---

## 6. Window Manager (blink)

### Layouts
| Layout | Description |
|--------|-------------|
| tile | Master + stack |
| grid | Equal-sized grid |
| quarter | 4-window fixed grid |
| float | Free floating |

### Keybinds

#### General
| Key | Action |
|-----|--------|
| Mod+Return | Terminal |
| Mod+d | App launcher |
| Mod+q | Close window |
| Mod+Shift+q | Quit WM |

#### Focus
| Key | Action |
|-----|--------|
| Mod+h/j/k/l | Focus direction |
| Mod+Tab | Cycle forward |
| Mod+Shift+Tab | Cycle backward |

#### Window
| Key | Action |
|-----|--------|
| Mod+f | Fullscreen |
| Mod+t | Toggle float |
| Mod+Space | Cycle layout |
| Mod+g | Toggle gaps |
| Mod+[ | Decrease master |
| Mod+] | Increase master |
| Mod+LeftClick | Move window |
| Mod+RightClick | Resize window |

#### Tags
| Key | Action |
|-----|--------|
| Mod+1-9 | View tag |
| Mod+Shift+1-9 | Move to tag |

#### Scratchpad
| Key | Action |
|-----|--------|
| Mod+` | Toggle scratchpad |

#### Move Operations
| Key | Action |
|-----|--------|
| Mod+Shift+c | Center window |
| Mod+Shift+f | Float to screen |
| Mod+Alt+h/j/k/l | Snap to halves |
| Mod+Shift+p | Pin window |

#### TUIs
| Key | Action |
|-----|--------|
| Mod+/ | Keybind viewer |
| Mod+Shift+c | Config TUI |
| Mod+Shift+p | Package manager |
| Mod+Shift+s | System actions |
| Mod+Shift+v | Network + Volume |
| Mod+Shift+n | Notifications |

#### System
| Key | Action |
|-----|--------|
| Mod+Shift+r | Reload config |
| Mod+l | Lock screen |

---

## 7. TUIs Overview

### blink-ctl (Config TUI)
```
Mod+Shift+c to launch
```
- Keybindings (add/edit/delete, categories)
- Tags (names, icons, layouts)
- Status Bar (position, widgets)
- Launcher (grid size, root apps)
- Compositor (picom settings)
- Notifications (dunst config)
- Appearance (theme, Wal)
- Mouse (enable/disable)

### blink-bar (Status Bar)
```
Auto-start with system
```
- Tags display
- Window title
- CPU/RAM/Disk
- Volume
- Network
- Datetime

### blink-launcher (App Launcher)
```
Mod+d to launch
```
- Grid view (6 columns)
- Search with ">" prefix
- App icons from XDG themes
- Modes: Apps, Commands, Root, System

### blink-keybinds (Keybind Viewer)
```
Mod+/ to launch
```
- Shows all keybinds
- Organized by category
- Read-only view

### blink-sys (Package Manager)
```
Mod+Shift+p to launch
```
- Search packages (pacman + Paru)
- Filters: All, Installed, Updates, AUR, Chaotic
- Multi-select queue
- Repo badges (core, extra, aur, chaotic)
- Mirror manager (rate-mirrors-bin)
- Chaotic-AUR installer
- User management

### blink-system (System Actions)
```
Mod+Shift+s to launch
```
- Lock screen
- Restart
- Shutdown
- Suspend
- Sleep
- Logout

### blink-controls (Network + Volume)
```
Mod+Shift+v to launch
```
- Tabs: Network, Volume
- Network: WiFi, Ethernet, Modem, VPN, DNS
- Volume: Output devices, Input, App volumes
- PipeWire + PulseAudio support

### blink-notifications (Notification History)
```
Mod+Shift+n to launch
```
- View notification history
- Clear notifications
- Filter by app

---

## 8. Implementation Order

### Phase 1: Foundation
1. Reorganize project to `/tui/` and `/libs/`
2. Fix build error (users crate version)
3. Create shared libraries:
   - blink-theme
   - blink-ipc
   - blink-config

### Phase 2: Window Manager
4. Implement blink WM with Penrose
5. Add layouts (tile, grid, quarter, float)
6. Implement keybinds
7. Add IPC server
8. Window rules system

### Phase 3: Core TUIs
9. blink-ctl (Config TUI)
10. blink-launcher (App Launcher)

### Phase 4: Utility TUIs
11. blink-keybinds
12. blink-sys
13. blink-system
14. blink-controls

### Phase 5: Integration
15. blink-notifications
16. blink-bar

### Phase 6: Polish
17. Theme consistency
18. Performance optimization
19. Testing

---

## 9. Arch Linux Package

### PKGBUILD
```bash
# Maintainer: Your Name
pkgname=blinkWM
pkgver=0.1.0
pkgrel=1
pkgdesc="Sleek, minimal X11 window manager"
arch=('x86_64')
license=('MIT')
depends=('rust' 'penrose' 'xorg-xinit' 'libx11' 'libxcb')
optdepends=('picom' 'dunst')
source=("$pkgname-$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$pkgname"
    cargo build --release
}

package() {
    cd "$pkgname"
    install -Dm755 target/release/blink "$pkgdir/usr/bin/blink"
    install -Dm755 target/release/blink-* "$pkgdir/usr/bin/"
    install -Dm644 config/blink.lua "$pkgdir/etc/xdg/blink/config.lua"
}
```

---

## 10. Key Design Decisions

| Decision | Choice |
|----------|--------|
| Framework | Penrose (Rust) |
| Backend | X11 only |
| Config Format | TOML |
| Config Location | ~/.config/blink/ |
| IPC | Unix socket (dynamic) |
| Theme | Unified (all TUIs) |
| Dependencies | Dynamic linking |
| Package | All-in-one (blinkWM) |

---

## 11. Performance Targets

| Metric | Target |
|--------|--------|
| WM startup | <500ms |
| TUIs startup | <200ms |
| Memory (WM only) | ~40MB |
| Memory (all TUIs) | ~100MB |

---

## 12. Visual Style

### Theme Colors
| Element | Color |
|---------|-------|
| Background | #1e1e2e |
| Foreground | #cdd6f4 |
| Accent | #89b4fa |
| Success | #a6e3a1 |
| Warning | #f9e2af |
| Error | #f38ba8 |
| Border | #313244 |

### UI Style
- Minimal borders
- JetBrains Mono font
- Subtle hover effects
- Rounded corners (8px)
- Consistent spacing

---

## 13. Configuration (All in TUI)

Every setting configurable through blink-ctl TUI:
- Keybindings
- Tags
- Status Bar
- Launcher
- Compositor
- Notifications
- Appearance
- Mouse
- System (autostart, tray)

No config files needed - TUI handles everything!

---

## Summary

BlinkWM is a complete, integrated desktop experience:
- ✅ Window Manager (tile, grid, quarter, float)
- ✅ App Launcher (rofi-based, grid view)
- ✅ Config TUI (everything configurable)
- ✅ Package Manager (pacman + Paru)
- ✅ System Actions (lock, reboot, etc.)
- ✅ Network + Volume Controls
- ✅ Notification History
- ✅ Status Bar
- ✅ Keybind Viewer
- ✅ IPC integrated
- ✅ Unified theme
- ✅ Arch package ready

All debloated, fast, and minimal!

---

# TODO LIST - Implementation Tasks

## Phase 1: Project Setup & Foundation

### 1.1 Reorganize Project Structure
- [ ] Create `/home/bhishma/wm2/libs/` directory
- [ ] Create `/home/bhishma/wm2/tui/` directory
- [ ] Move existing TUI crates to `/tui/`
- [ ] Update workspace Cargo.toml with new paths
- [ ] Verify project compiles after move

### 1.2 Fix Build Error
- [ ] Identify the users crate version issue in Cargo.toml
- [ ] Update users crate to version 3.x or compatible
- [ ] Test build with `cargo build`

### 1.3 Create Shared Library: blink-theme
- [ ] Create `/home/bhishma/wm2/libs/blink-theme/Cargo.toml`
- [ ] Implement Theme struct with colors
- [ ] Add load from TOML function
- [ ] Add default theme values
- [ ] Test library compiles

### 1.4 Create Shared Library: blink-ipc
- [ ] Create `/home/bhishma/wm2/libs/blink-ipc/Cargo.toml`
- [ ] Implement IPC client struct
- [ ] Implement IPC server struct
- [ ] Add event subscription system
- [ ] Add socket connection handling
- [ ] Test library compiles

### 1.5 Create Shared Library: blink-config
- [ ] Create `/home/bhishma/wm2/libs/blink-config/Cargo.toml`
- [ ] Implement Config struct
- [ ] Add TOML read/write functions
- [ ] Add config file watching
- [ ] Add hot reload support
- [ ] Test library compiles

---

## Phase 2: Window Manager (blink)

### 2.1 Core WM Setup
- [ ] Setup Penrose with X11 backend
- [ ] Configure modkey (Mod4/Super)
- [ ] Add basic keybinds
- [ ] Test WM starts without errors

### 2.2 Layouts Implementation
- [ ] Implement tile layout (master + stack)
- [ ] Implement grid layout (equal-sized)
- [ ] Implement quarter layout (4-window)
- [ ] Implement float layout
- [ ] Add layout cycling keybind

### 2.3 Window Management
- [ ] Add floating window toggle (Mod+t)
- [ ] Add fullscreen toggle (Mod+f)
- [ ] Add mouse move (Mod+LeftClick)
- [ ] Add mouse resize (Mod+RightClick)
- [ ] Implement smart gaps
- [ ] Implement smart borders

### 2.4 Tags System
- [ ] Configure 9 tags/workspaces
- [ ] Add tag switching keybinds (Mod+1-9)
- [ ] Add tag move keybinds (Mod+Shift+1-9)
- [ ] Add tag indicator visual feedback

### 2.5 Scratchpad
- [ ] Implement scratchpad (Mod+`)
- [ ] Add show/hide functionality

### 2.6 Window Operations
- [ ] Add center window (Mod+Shift+c)
- [ ] Add float to screen (Mod+Shift+f)
- [ ] Add snap to halves (Mod+Alt+hjkl)
- [ ] Add pin window (Mod+Shift+p)
- [ ] Add gaps toggle (Mod+g)

### 2.7 Window Rules
- [ ] Implement rules parsing from config
- [ ] Add rule: app → tag
- [ ] Add rule: app → layout
- [ ] Add rule: app → float

### 2.8 IPC Server
- [ ] Add IPC server to WM
- [ ] Implement event: tag_changed
- [ ] Implement event: focus_changed
- [ ] Implement event: layout_changed
- [ ] Implement event: client_list
- [ ] Implement event: window_open
- [ ] Implement event: window_close

### 2.9 Border Rendering
- [ ] Add focused border color
- [ ] Add unfocused border color
- [ ] Add urgent border color
- [ ] Configure border width

### 2.10 Focus Behavior
- [ ] Implement focus follows mouse
- [ ] Add click to focus option
- [ ] Add autoraise option

### 2.11 Autostart
- [ ] Implement autostart from config
- [ ] Run commands on WM startup

---

## Phase 3: blink-ctl (Config TUI)

### 3.1 Basic Structure
- [ ] Setup ratatui-based TUI
- [ ] Add main menu layout
- [ ] Add navigation (keyboard + mouse)

### 3.2 Keybindings Section
- [ ] Display current keybinds
- [ ] Add keybind editing
- [ ] Add keybind deletion
- [ ] Add keybind addition
- [ ] Add category management

### 3.3 Tags Section
- [ ] Display tags
- [ ] Edit tag names
- [ ] Edit tag icons
- [ ] Edit tag layouts

### 3.4 Status Bar Section
- [ ] Configure bar position (top/bottom)
- [ ] Enable/disable widgets
- [ ] Configure widget order

### 3.5 Launcher Section
- [ ] Configure grid columns (1-8)
- [ ] Configure icon size
- [ ] Manage root apps list
- [ ] Configure search prefix

### 3.6 Compositor Section
- [ ] General: enable, backend, vsync
- [ ] Rounded: enable, radius, exclusions
- [ ] Blur: enable, backend, strength
- [ ] Animations: enable, duration
- [ ] Exclusions: shadow, fade lists

### 3.7 Notifications Section
- [ ] General: enable, timeout
- [ ] Appearance: position, width, colors
- [ ] Rules: per-app settings
- [ ] DND: enable, auto-rules

### 3.8 Appearance Section
- [ ] Presets: Nord, Dracula, etc.
- [ ] Wal: backend selector, auto-sync
- [ ] Per-app: configure colors per app
- [ ] Save/load colorschemes

### 3.9 Settings Section
- [ ] Mouse: enable/disable
- [ ] Startup: bar, compositor

### 3.10 Config Persistence
- [ ] Write config to TOML
- [ ] Read config on startup
- [ ] Add hot reload trigger

---

## Phase 4: blink-launcher (App Launcher)

### 4.1 Basic Structure
- [ ] Setup GTK/rofi-based launcher
- [ ] Grid view layout (6 columns)
- [ ] Search bar at bottom with ">" prefix

### 4.2 App Discovery
- [ ] Parse .desktop files
- [ ] Load app icons from XDG
- [ ] Display app name below icon

### 4.3 Search Functionality
- [ ] Fuzzy search as you type
- [ ] Filter results in real-time

### 4.4 Modes
- [ ] Apps mode (default)
- [ ] Commands mode (custom commands)
- [ ] Root mode (apps requiring pkexec)
- [ ] System mode (system actions)

### 4.5 Keybinds
- [ ] Arrow key navigation
- [ ] Enter to launch
- [ ] Escape to close
- [ ] Tab to switch modes

### 4.6 Root App Support
- [ ] Mark root apps with lock icon
- [ ] Launch with pkexec
- [ ] Configure root apps list

### 4.7 IPC Integration
- [ ] Connect to WM IPC
- [ ] Spawn apps via WM

---

## Phase 5: blink-keybinds (Keybind Viewer)

### 5.1 Basic Structure
- [ ] Setup ratatui TUI
- [ ] Display all keybinds

### 5.2 Categories
- [ ] Group by category
- [ ] Category tabs (All, General, Window, TUI, System)

### 5.3 Display
- [ ] Show key, action, command
- [ ] Read-only view

### 5.4 Navigation
- [ ] Tab to switch categories
- [ ] Arrow keys to navigate

---

## Phase 6: blink-sys (Package Manager)

### 6.1 Basic Structure
- [ ] Setup ratatui TUI
- [ ] Package search input

### 6.2 Package Search
- [ ] Search via pacman
- [ ] Search via Paru (AUR)
- [ ] Display results with repo badges

### 6.3 Repo Badges
- [ ] core (green)
- [ ] extra (blue)
- [ ] aur (purple)
- [ ] chaotic (red)

### 6.4 Filters
- [ ] All packages
- [ ] Installed
- [ ] Updates available
- [ ] AUR only
- [ ] Chaotic only

### 6.5 Multi-select Queue
- [ ] Space to select
- [ ] Queue display
- [ ] Install queued
- [ ] Remove selected

### 6.6 Mirror Manager
- [ ] Run rate-mirrors-bin
- [ ] Display mirror status
- [ ] Save mirrorlist

### 6.7 Chaotic-AUR
- [ ] Check if installed
- [ ] Install script
- [ ] Update
- [ ] Remove

### 6.8 User Management
- [ ] List users
- [ ] Add user
- [ ] Delete user
- [ ] Edit user (requires root)

---

## Phase 7: blink-system (System Actions)

### 7.1 Basic Structure
- [ ] Setup ratatui TUI
- [ ] Grid layout for actions

### 7.2 Actions
- [ ] Lock screen (slock/i3lock)
- [ ] Restart (systemctl reboot)
- [ ] Shutdown (systemctl poweroff)
- [ ] Suspend (systemctl suspend)
- [ ] Sleep (systemctl sleep)
- [ ] Logout (kill WM)

### 7.3 Keybinds
- [ ] Enter to execute
- [ ] Arrow keys to navigate

---

## Phase 8: blink-controls (Network + Volume)

### 8.1 Basic Structure
- [ ] Setup ratatui TUI
- [ ] Tab navigation (Network/Volume)

### 8.2 Network Tab
- [ ] WiFi: list, connect, disconnect
- [ ] Ethernet: list, toggle
- [ ] Modem: detect, connect
- [ ] VPN: WireGuard, OpenVPN
- [ ] DNS: preset, custom

### 8.3 Volume Tab
- [ ] Detect backend (PipeWire/PulseAudio)
- [ ] Output device list
- [ ] Input device list
- [ ] Master volume control
- [ ] Mute toggle
- [ ] App volumes (if possible)

### 8.4 IPC Integration
- [ ] Query WM for window info
- [ ] Show focused app volume

---

## Phase 9: blink-notifications (Notification History)

### 9.1 Basic Structure
- [ ] Setup ratatui TUI
- [ ] Read from dunst history

### 9.2 Display
- [ ] List notifications
- [ ] Show app, message, time
- [ ] Filter by app

### 9.3 Actions
- [ ] Clear single
- [ ] Clear all
- [ ] Open notification (if applicable)

---

## Phase 10: blink-bar (Status Bar)

### 10.1 Basic Structure
- [ ] Setup X11 bar rendering
- [ ] Position config (top/bottom)

### 10.2 Widgets
- [ ] Tags widget
- [ ] Title widget
- [ ] CPU widget
- [ ] RAM widget
- [ ] Disk widget
- [ ] Volume widget
- [ ] Network widget
- [ ] Datetime widget

### 10.3 IPC Subscription
- [ ] Subscribe to tag_changed
- [ ] Subscribe to focus_changed
- [ ] Subscribe to layout_changed
- [ ] Update on events

### 10.4 Theme Integration
- [ ] Use blink-theme for colors
- [ ] Follow config colors

---

## Phase 11: Integration & Polish

### 11.1 Theme Consistency
- [ ] All TUIs use blink-theme
- [ ] Consistent styling
- [ ] Consistent fonts

### 11.2 Performance
- [ ] Optimize startup time
- [ ] Optimize memory usage
- [ ] Test with cargo bench if needed

### 11.3 Testing
- [ ] Test all keybinds
- [ ] Test all TUIs
- [ ] Test IPC communication
- [ ] Test config persistence

### 11.4 Documentation
- [ ] Add README.md
- [ ] Add installation instructions
- [ ] Add usage instructions

---

## Phase 12: Package Creation

### 12.1 PKGBUILD
- [ ] Create package/PKGBUILD
- [ ] Add proper depends
- [ ] Add optdepends
- [ ] Test build

### 12.2 Package Contents
- [ ] blink binary
- [ ] blink-launcher binary
- [ ] blink-ctl binary
- [ ] blink-bar binary
- [ ] blink-keybinds binary
- [ ] blink-sys binary
- [ ] blink-system binary
- [ ] blink-controls binary
- [ ] blink-notifications binary

### 12.3 Installation
- [ ] Test install on clean system
- [ ] Verify all binaries work

---

## Summary Checklist

- [ ] Phase 1: Foundation (5 tasks)
- [ ] Phase 2: WM (11 tasks)
- [ ] Phase 3: blink-ctl (10 tasks)
- [ ] Phase 4: blink-launcher (7 tasks)
- [ ] Phase 5: blink-keybinds (4 tasks)
- [ ] Phase 6: blink-sys (8 tasks)
- [ ] Phase 7: blink-system (3 tasks)
- [ ] Phase 8: blink-controls (4 tasks)
- [ ] Phase 9: blink-notifications (3 tasks)
- [ ] Phase 10: blink-bar (4 tasks)
- [ ] Phase 11: Integration (4 tasks)
- [ ] Phase 12: Package (3 tasks)

**Total: 66 tasks**
