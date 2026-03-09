# BlinkWM Development Plan

## Project Overview

**Name**: BlinkWM  
**Type**: X11 Window Manager for Arch Linux  
**Language**: Rust  
**Architecture**: Hybrid tiling/floating window manager  

## Key Technical Decisions

| Decision | Value | Rationale |
|----------|-------|-----------|
| Display Server | X11 only | User choice - simpler scope |
| Language | Rust | Memory safety for system-level code |
| X11 Bindings | xcb crate | Safe Rust bindings, actively maintained |
| WM Style | Hybrid | User switches between tiling/floating |
| Config Format | TOML | Familiar to Arch users, good serde support |
| Keybindings | i3-style | Mod+key pattern, familiar to users |
| Status Bar | Separate process | Can restart independently |
| IPC | i3-compatible | JSON over UNIX socket |
| Packaging | PKGBUILD | Native Arch/AUR |
| Startup | xsession | Standard X session |

## Scope

### IN (Included)
- X11 window management (tiling + floating)
- Multi-monitor support (via RandR)
- Status bar (separate blinkwm-bar process)
- System tray support
- i3-compatible IPC socket
- TOML configuration file
- Workspace management
- Keybinding system

### OUT (Deferred)
- Wayland support (future phase)
- Built-in compositor effects (future phase)

---

## Core WM Features

### 1. Gaps & Borders

#### Smart Gaps
- **Outer Gaps**: Gap between windows and screen edge
- **Inner Gaps**: Gap between adjacent windows
- **Smart Gaps**: Hide gaps when only one window visible

#### Smart Borders
- **Border Width**: Configurable (default 2px)
- **Smart Borders**: Hide borders when only one window
- **Border Colors**: Different colors per focus state

#### Border Color States
| State | Default Color | When |
|-------|---------------|------|
| **Focused** | #007AFF (Blue) | Currently selected window |
| **Unfocused** | #3D3D3D (Gray) | All other windows |
| **Urgent** | #FF3B30 (Red) | Window requires attention |

---

### 2. Window Rules

Match windows and apply automatic settings:

```toml
[[window_rule]]
    class = "firefox"
    workspace = 2

[[window_rule]]
    class = "Rofi"
    floating = true
    borderless = true
```

**Match Criteria**: class, instance, title (regex), role, type

---

### 3. Layouts

| Layout | Description |
|--------|-------------|
| **Vertical Split** | Split vertically (left/right) |
| **Horizontal Split** | Split horizontally (top/bottom) |
| **Stacking** | Stack windows like traditional WM |
| **Fullscreen** | Fullscreen mode |

---

### 4. Workspaces

- Default: 10 workspaces (1-10)
- Named workspaces support
- **Bar shows occupied only** (user preference)
- Multi-monitor: independent workspaces per monitor

---

### 5. Scratchpad

- Dropdown terminal support
- Default: MOD+SHIFT+Return to toggle

---

### 6. Window Operations

- Focus (follows mouse + click both configurable)
- Move, resize
- Swap windows
- Toggle floating/fullscreen

---

### 7. Preselection

Pre-select split direction before opening window:
- MOD+ALT+V → preselect vertical
- MOD+ALT+H → preselect horizontal

---

### 8. Mouse Bindings

```toml
[mouse]
focus_follows_mouse = true
mouse drag = true  # drag floating windows from anywhere
```

---

### Default Keybindings

#### Window Management
| Keybind | Action |
|---------|--------|
| `MOD+J` | Focus down |
| `MOD+K` | Focus up |
| `MOD+H` | Focus left |
| `MOD+L` | Focus right |
| `MOD+SHIFT+J` | Move window down |
| `MOD+SHIFT+K` | Move window up |
| `MOD+SHIFT+H` | Move window left |
| `MOD+SHIFT+L` | Move window right |
| `MOD+ALT+J` | Swap with window below |
| `MOD+ALT+K` | Swap with window above |
| `MOD+ALT+H` | Swap with window left |
| `MOD+ALT+L` | Swap with window right |

#### Window Operations
| Keybind | Action |
|---------|--------|
| `MOD+F` | Toggle floating |
| `MOD+SHIFT+F` | Toggle fullscreen |
| `MOD+Q` | Close window |
| `MOD+SHIFT+SPACE` | Toggle float ↔ tiled |
| `MOD+SHIFT+Q` | Force close window |
| `MOD+MINUS` | Minimize to scratchpad |
| `MOD+E` | Toggle sticky (all workspaces) |

#### Layout & Tiling
| Keybind | Action |
|---------|--------|
| `MOD+SPACE` | Cycle layout (V → H → S → V) |
| `MOD+ALT+V` | Split vertical |
| `MOD+ALT+H` | Split horizontal |
| `MOD+T` | Set layout: tiled |
| `MOD+S` | Set layout: stacking |
| `MOD+M` | Set layout: monocle (fullscreen) |
| `MOD+A` | Focus parent container |
| `MOD+SHIFT+A` | Focus child container |

#### Workspaces
| Keybind | Action |
|---------|--------|
| `MOD+1-9,0` | Switch to workspace 1-10 |
| `MOD+SHIFT+1-9,0` | Move window to workspace |
| `MOD+TAB` | Next workspace |
| `MOD+SHIFT+TAB` | Previous workspace |
| `MOD+[` | Next workspace |
| `MOD+]` | Previous workspace |
| `MOD+F1-F10` | Go to workspace (alternative) |

#### Applications
| Keybind | Action |
|---------|--------|
| `MOD+ENTER` | Launch terminal |
| `MOD+D` | App launcher (blinkwm-dmenu) |
| `MOD+SHIFT+ENTER` | Scratchpad terminal |
| `MOD+ALT+ENTER` | Floating terminal |

#### Resize Mode
| Keybind | Action |
|---------|--------|
| `MOD+R` | Enter resize mode |
| `↑↓←→` | Resize (in resize mode) |
| `H/J/K/L` | Resize (in resize mode) |
| `ENTER` | Confirm resize |
| `ESC` | Cancel resize |

#### System
| Keybind | Action |
|---------|--------|
| `MOD+SHIFT+R` | Reload config |
| `MOD+SHIFT+E` | Exit BlinkWM |
| `MOD+SHIFT+C` | Restart BlinkWM |
| `MOD+L` | Lock screen |
| `MOD+X` | Show keybindings list |

#### Preselection
| Keybind | Action |
|---------|--------|
| `MOD+ALT+V` | Preselect vertical split |
| `MOD+ALT+H` | Preselect horizontal split |
| `MOD+ALT+1-9` | Preselect workspace |
| `MOD+ALT+SPACE` | Clear preselection |

#### Mouse Keybinds (optional)
| Keybind | Action |
|---------|--------|
| `MOD+Left Click` | Move window |
| `MOD+Right Click` | Resize window |
| `MOD+Middle Click` | Raise window |
| `Left Click` | Focus window |
| `Scroll on workspace` | Switch workspace |

---

### Complete Default Keybinding List

```
# Focus
MOD+J                 focus down
MOD+K                 focus up
MOD+H                 focus left
MOD+L                 focus right
MOD+LEFT              focus left
MOD+RIGHT             focus right
MOD+UP                focus up
MOD+DOWN              focus down

# Move
MOD+SHIFT+J           move down
MOD+SHIFT+K           move up
MOD+SHIFT+H           move left
MOD+SHIFT+L           move right

# Swap
MOD+ALT+J             swap down
MOD+ALT+K             swap up
MOD+ALT+H             swap left
MOD+ALT+L             swap right

# Window
MOD+F                 toggle floating
MOD+SHIFT+F           toggle fullscreen
MOD+Q                 close window
MOD+SHIFT+SPACE       toggle float/tiled
MOD+SHIFT+Q           force close
MOD+MINUS             to scratchpad
MOD+E                 sticky toggle

# Layout
MOD+SPACE             cycle layout
MOD+ALT+V             split vertical
MOD+ALT+H             split horizontal
MOD+T                 tiled layout
MOD+S                 stacking layout
MOD+M                 monocle layout

# Container
MOD+A                 focus parent
MOD+SHIFT+A           focus child

# Workspaces
MOD+1-9,0            workspace 1-10
MOD+SHIFT+1-9,0      move to workspace
MOD+TAB               next workspace
MOD+SHIFT+TAB        prev workspace
MOD+COMMA            next workspace
MOD+PERIOD           prev workspace

# Apps
MOD+ENTER             terminal
MOD+SHIFT+ENTER      scratchpad
MOD+D                 dmenu/launcher

# Resize
MOD+R                 resize mode
  ↑↓←→               resize
  ENTER               confirm
  ESC                 cancel

# System
MOD+SHIFT+R           reload config
MOD+SHIFT+E           exit
MOD+SHIFT+C           restart
MOD+X                 show keybindings
```

---

### Missing (Self-Corrected)
- EWMH support - need for external tool compatibility
- Config runtime reload - added to Phase 5

---

## Implementation Phases

### Phase 1: Project Foundation

#### 1.1 Initialize Rust Project
```bash
cargo new --bin blinkwm
```
- Add dependencies:
  - `xcb` - X11 protocol bindings
  - `calloop` - Event loop
  - `serde`, `toml` - Config parsing
  - `tracing`, `tracing-subscriber` - Logging
  - `regex` - Window matching
  - `signal-hook` - Signal handling
  - `swayipc-types` - IPC types (optional, can implement manually)
- Set up logging with tracing crate
- Create basic project structure

#### 1.2 X11 Connection Setup
- Implement XCB connection initialization
- Handle display connection errors gracefully
- Set up basic event loop with calloop
- Window manager mode: override-redirect

#### 1.3 Basic Window Handling
- Catch windows as they map
- Track window list (Vec of window IDs)
- Handle DestroyNotify events
- Basic window positioning

### Phase 2: Core Window Management

#### 2.1 Window Data Structure
```rust
struct ManagedWindow {
    id: xcb::x::Window,
    x: i16, y: i16,
    width: u16, height: u16,
    border_width: u32,
    floating: bool,
    workspace: usize,
    // EWMH properties
    class: String,
    instance: String,
    title: String,
    // ... other state
}
```

#### 2.2 Frame Windows
- Create frame windows for managed clients
- Handle resize/move events
- Border rendering
- Title bar (optional, or use client title)

#### 2.3 Tiling Engine
- Vertical/horizontal split layouts
- Container/workspace data structures
- Layout calculation functions
- Focus management

#### 2.4 Floating Mode
- Allow free positioning
- Resize handles
- Switch between tiling/floating

### Phase 3: Workspaces & Multi-Monitor

#### 3.1 Workspace Management
- Multiple workspaces (default: 10)
- Workspace switching via keybindings
- Persist workspace assignments

#### 3.2 Multi-Monitor (RandR)
- Detect monitors via RandR extension
- Handle monitor connect/disconnect
- Workspace-to-monitor mapping
- Moving windows between monitors

### Phase 4: Status Bar

#### 4.1 Bar Process
- Separate blinkwm-bar binary
- IPC communication with main WM
- Display: workspace names, window title, layout
- Customizable via config

#### 4.2 IPC Protocol
- UNIX socket at `$XDG_RUNTIME_DIR/blinkwm/ipc-socket.%p`
- i3-compatible message format:
  - Magic string: "i3-ipc"
  - Message type (u32): 0=command, 1=workspaces, 2=subscribe, etc.
  - JSON payload
- Use swayipc-types crate for serialization

### Phase 5: Configuration System

#### 5.1 Config File
- Location: `~/.config/blinkwm/config.toml`
- Keybindings section
- Workspace assignments
- Startup programs
- Window rules

#### 5.2 Window Rules (with EWMH)
```toml
[window-rules]
# Match by: class, instance, title (regex), role, type
[[rule]]
    class = "Firefox"
    floating = false
    workspace = 2

[[rule]]
    class = "Rofi"
    floating = true
    borderless = true
    sticky = true

[[rule]]
    title = ".*popup.*"
    floating = true
    width = 600
    height = 400

[[rule]]
    instance = "float"
    floating = true
    position = "center"
```

#### 5.3 Gaps & Borders Config
```toml
[gaps]
inner = 10
outer = 15
smart_gaps = true

[border]
width = 2
smart_borders = "hide"  # "hide" | "always"

[colors]
border_focused = "#007AFF"
border_unfocused = "#3D3D3D"
border_urgent = "#FF3B30"
border_background = "#1E1E1E"
```

#### 5.4 Mouse Config
```toml
[mouse]
# Focus
focus_follows_mouse = true       # Focus follows cursor
focus_on_click = true            # Click to focus

# Dragging (floating windows)
drag_floating = true             # Drag anywhere on window
drag_tiling = false              # Drag tiling windows (optional)

# Resizing (floating windows)
resize_on_corner = true          # Resize from corners/edges

# Mouse bindings
[mouse.bindings]
# Format: "MOD+Button" = action
"MOD+Button1" = "move"          # Move floating window
"MOD+Button2" = "resize"         # Resize floating window
"MOD+Button3" = "raise"         # Bring to front
"Button1" = "focus"             # Click to focus
"Button3" = "menu"              # Right-click menu
```

#### 5.5 Mouse Move/Resize Behavior
| Action | Trigger | Description |
|--------|---------|-------------|
| **Move** | MOD+Left Click Drag | Move floating window |
| **Resize** | MOD+Right Click Drag | Resize floating window |
| **Resize Edge** | Move to edge | Resize from window edges |
| **Focus** | Left Click | Focus window |
| **Raise** | MOD+Middle Click | Bring window to front |

#### 5.6 Resize Directions
```
┌──────────────┐
│ NW   N   NE │
│              │
│  W   ◉   E  │  ◉ = center (move)
│              │
│ SW   S   SE │
└──────────────┘
Move cursor to any edge/corner to resize
```

#### 5.7 Runtime Config Reload
- Signal-based reload (SIGHUP)
- Re-read config file without restart
- Apply keybinding changes

### Phase 6: Keybindings & IPC

#### 6.1 EWMH Support
- Set _NET_SUPPORTED property on root window
- Report window list via _NET_CLIENT_LIST
- Workspace names via _NET_DESKTOP_NAMES
- Active window via _NET_ACTIVE_WINDOW
- Window state hints (_NET_WM_STATE)
- Required for polybar/i3status compatibility

#### 6.2 Input Handling
- XCB input extension for keyboard
- XKB for keycode translation
- Mod key handling (Mod4 typically)

#### 6.2 Default Keybindings
```
Mod+Enter     - Terminal
Mod+d        - Dmenu/app launcher
Mod+j/k      - Focus left/down
Mod+h/l      - Focus right/up
Mod+Shift+j  - Move window left
Mod+Shift+k  - Move window down
Mod+Shift+h  - Move window right
Mod+Shift+l  - Move window up
Mod+f        - Toggle floating
Mod+1-9      - Workspace 1-9
Mod+Shift+1-9 - Move window to workspace
Mod+q        - Close window
Mod+Space    - Layout toggle
```

### Phase 7: System Integration

#### 7.1 Startup (xsession)
- Create `/usr/share/xsessions/blinkwm.desktop`
- Session script for initialization

#### 7.2 System Tray
- Support for tray icons (optional)
- Follows system tray specification

#### 7.3 PKGBUILD
- Package for AUR
- Dependencies: libxcb, rust, cargo

---

## Technical Notes

### XCB Patterns (from research)
1. **Event loop**: Use calloop for async event handling
2. **Connection**: Handle connection errors, display detection
3. **Extensions**: Enable randr, xinput, xkb as needed
4. **Windows**: Use override-redirect for frame windows
5. **Focus**: Maintain focus state manually (X11 doesn't track)

### IPC Implementation
- Socket path: `$XDG_RUNTIME_DIR/blinkwm/ipc-socket.%p`
- Fallback: `/tmp/blinkwm-%u/ipc-socket.%p`
- Magic string: "i3-ipc" (i3 compatibility)
- Message types: 0=command, 1=workspaces, 2=subscribe, 3=outputs, etc.
- Reference: swayipc-types crate for types

### Common Pitfalls
1. Event loop hangs with no windows - need dummy window or socket watch
2. XCB extensions not announced - need runtime detection
3. Focus stealing - need explicit focus management
4. Multi-monitor RandR - handle hotplug gracefully

---

## Acceptance Criteria

### Phase 1 (Foundation)
- [ ] Project compiles with `cargo build`
- [ ] Connects to X11 display
- [ ] Event loop processes X11 events
- [ ] Logs to file/tracing

### Phase 2 (Core WM)
- [ ] Windows are framed with borders
- [ ] Tiling layout positions windows
- [ ] Floating mode allows free movement
- [ ] Focus can be changed

### Phase 3 (Workspaces)
- [ ] 10 workspaces available
- [ ] Switching workspaces works
- [ ] Multiple monitors detected
- [ ] Windows move between monitors

### Phase 4 (Bar)
- [ ] Bar shows workspace info
- [ ] IPC socket responds to queries
- [ ] Can integrate with polybar/i3status

### Phase 5 (Config)
- [ ] TOML config loads on startup
- [ ] Keybindings from config work
- [ ] Startup programs launch

### Phase 6 (Keybindings)
- [ ] All default keybindings functional
- [ ] Window close works
- [ ] Layout toggle works

### Phase 7 (Integration)
- [ ] xsession launches properly
- [ ] PKGBUILD builds successfully
- [ ] Installs and runs on Arch Linux

---

## File Structure

```
blinkwm/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── x11/
│   │   ├── mod.rs
│   │   ├── connection.rs
│   │   ├── events.rs
│   │   └── extensions.rs
│   ├── wm/
│   │   ├── mod.rs
│   │   ├── window.rs
│   │   ├── workspace.rs
│   │   ├── layout.rs
│   │   └── tiling.rs
│   ├── config/
│   │   ├── mod.rs
│   │   └── keybindings.rs
│   ├── ipc/
│   │   ├── mod.rs
│   │   └── socket.rs
│   └── bar/
│       └── main.rs
├── data/
│   └── blinkwm.desktop
└── PKGBUILD
```

---

## QA Scenarios

| Scenario | Test |
|----------|------|
| Single window | Launch terminal, verify framed |
| Tiling | Open 3 windows, verify split layout |
| Floating | Toggle floating, resize window |
| Workspace switch | Mod+1, Mod+2 - verify windows move |
| Multi-monitor | Connect second monitor, move window |
| Config reload | Edit config, restart - verify changes |
| IPC query | `i3-msg` style query works |
| Bar refresh | Close/reopen bar process |
| Close window | Mod+q closes focused window |
| Startup | Login with xsession, WM launches |

---

## Notes for Implementer

- Use `calloop` crate for event loop (standard in Rust X11 apps)
- Reference `rumpl/miniwm` tutorial for basic patterns
- Reference `i3/sway` source for IPC details
- Test in Xephyr for safe development
- Start with minimal (Phase 1-2) before adding features
