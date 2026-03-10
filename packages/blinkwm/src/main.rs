use std::sync::Arc;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;
use calloop::{generic::Generic, EventLoop, Interest, Mode};
use tracing::{info, error};

// 1. Atom Management for EWMH/ICCCM
x11rb::atom_manager! {
    pub Atoms: AtomsCookie {
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
        _NET_WM_NAME,
        _NET_SUPPORTED,
        _NET_ACTIVE_WINDOW,
        _NET_WM_WINDOW_TYPE,
        _NET_WM_WINDOW_TYPE_DOCK,
        _NET_WM_WINDOW_TYPE_TOOLBAR,
        _NET_WM_WINDOW_TYPE_MENU,
        _NET_WM_WINDOW_TYPE_UTILITY,
        _NET_WM_WINDOW_TYPE_SPLASH,
        _NET_WM_WINDOW_TYPE_DIALOG,
        _NET_WM_WINDOW_TYPE_NORMAL,
        UTF8_STRING,
    }
}

mod wm;
mod config;
mod ipc;

use wm::{ManagedWindow, Layout, Workspace};
use config::Config;
use ipc::IpcManager;

// 2. Global State Structure
struct BlinkState {
    conn: Arc<RustConnection>,
    root: Window,
    #[allow(dead_code)]
    atoms: Atoms,
    config: Config,
    ipc_manager: Option<IpcManager>,
    managed_windows: Vec<ManagedWindow>,
    workspaces: Vec<Workspace>,
    active_workspace: usize, // Index into workspaces (0-9)
    screen_width: u16,
    screen_height: u16,
}

fn main() -> anyhow::Result<()> {
    // ... rest of main initialization ...
    let (conn, screen_num) = x11rb::connect(None)?;
    let conn = Arc::new(conn);
    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;
    let (sw, sh) = (screen.width_in_pixels, screen.height_in_pixels);

    let atoms = Atoms::new(conn.as_ref())?.reply()?;

    // ... takeover code ...
    let wa = ChangeWindowAttributesAux::default()
        .event_mask(EventMask::SUBSTRUCTURE_REDIRECT | 
                    EventMask::SUBSTRUCTURE_NOTIFY | 
                    EventMask::STRUCTURE_NOTIFY |
                    EventMask::PROPERTY_CHANGE);
    
    if let Err(e) = conn.change_window_attributes(root, &wa)?.check() {
        error!("Failed to take over as Window Manager. Is another WM running? {:?}", e);
        return Err(e.into());
    }

    // Initialize 10 workspaces
    let mut workspaces = Vec::new();
    for i in 1..=10 {
        workspaces.push(Workspace::new(i, None));
    }

    let mut event_loop: EventLoop<BlinkState> = EventLoop::try_new()?;
    let handle = event_loop.handle();
    // IPC setup
    let ipc = IpcManager::new()?;
    
    let mut state = BlinkState {
        conn: conn.clone(),
        root,
        atoms,
        config: Config::load(),
        ipc_manager: Some(ipc.clone()),
        managed_windows: Vec::new(),
        workspaces,
        active_workspace: 0,
        screen_width: sw,
        screen_height: sh,
    };

    // Register X11 Connection as an Event Source
    use std::os::unix::io::{AsFd, BorrowedFd, AsRawFd};
    let raw_fd = conn.stream().as_raw_fd();
    let fd = unsafe { BorrowedFd::borrow_raw(raw_fd) };
    handle.insert_source(
        Generic::new(fd, Interest::READ, Mode::Level),
        |_, _, state| {
            while let Some(event) = state.conn.poll_for_event().ok().flatten() {
                handle_event(state, event);
            }
            Ok(calloop::PostAction::Continue)
        },
    )?;

    // Register IPC Listener
    let ipc_fd = unsafe { BorrowedFd::borrow_raw(ipc.listener().as_raw_fd()) };
    handle.insert_source(
        Generic::new(ipc_fd, Interest::READ, Mode::Level),
        move |_, _, state| {
            if let Some(stream) = state.ipc_manager.as_ref().and_then(|i| i.accept()) {
                ipc::handle_ipc_client(stream, state);
            }
            Ok(calloop::PostAction::Continue)
        }
    )?;

    setup_keybindings(&state)?;

    if state.config.compositor.enabled {
        let _ = start_compositor(&state.config);
    }

    // 6. Start the Event Loop
    info!("BlinkWM event loop started.");
    loop {
        event_loop.dispatch(None, &mut state)?;
    }
}

fn handle_event(state: &mut BlinkState, event: x11rb::protocol::Event) {
    match event {
        x11rb::protocol::Event::MapRequest(e) => {
            info!("MapRequest for window: 0x{:x}", e.window);
            
            if state.managed_windows.iter().any(|w| w.id == e.window) {
                return;
            }

            if let Ok(mut managed) = frame(state, e.window) {
                // Apply Rules
                let class = get_window_class(&state.conn, e.window);
                let mut target_ws = state.workspaces[state.active_workspace].id;
                
                for rule in &state.config.rules {
                    if let (Some(rule_class), Some(actual_class)) = (&rule.class, &class) {
                        if rule_class == actual_class {
                            if let Some(ws) = rule.workspace { target_ws = ws; }
                            if let Some(f) = rule.floating { managed.floating = f; }
                        }
                    }
                }

                managed.workspace = target_ws;
                state.managed_windows.push(managed);
                let _ = apply_layout(state);
            }
        }
        x11rb::protocol::Event::DestroyNotify(e) => {
            info!("DestroyNotify for window: 0x{:x}", e.window);
            state.managed_windows.retain(|w| w.id != e.window);
            let _ = apply_layout(state);
        }
        x11rb::protocol::Event::UnmapNotify(e) => {
            info!("UnmapNotify for window: 0x{:x}", e.window);
        }
        x11rb::protocol::Event::KeyPress(e) => {
            handle_key_press(state, e);
        }
        _ => {}
    }
}

fn handle_key_press(state: &mut BlinkState, event: KeyPressEvent) {
    let mod_mask = event.state;
    let keycode = event.detail;
    let mut action_to_execute = None;

    for (bind, action) in &state.config.keybindings {
        if let Some((target_mask, target_keysym)) = parse_keybind(bind) {
            let target_keycode = get_keycode_from_keysym(state, target_keysym);
            if target_keycode == keycode && u16::from(mod_mask) == u16::from(target_mask) {
                action_to_execute = Some(action.clone());
                break;
            }
        }
    }

    if let Some(action) = action_to_execute {
        let _ = execute_action(state, &action);
    }
}

fn execute_action(state: &mut BlinkState, action: &str) -> anyhow::Result<()> {
    info!("Executing action: {}", action);
    match action {
        "terminal" => {
            std::process::Command::new("alacritty").spawn().or_else(|_| {
                std::process::Command::new("xterm").spawn()
            })?;
        }
        "launcher" => {
            std::process::Command::new("blinkwm-dmenu").spawn().or_else(|_| {
                // If not installed in PATH, try local build path for testing
                std::process::Command::new("./target/debug/blinkwm-dmenu").spawn()
            })?;
        }
        "next_layout" => {
            let next = match state.workspaces[state.active_workspace].layout {
                Layout::Vertical => Layout::Horizontal,
                Layout::Horizontal => Layout::Stacking,
                Layout::Stacking => Layout::Vertical,
            };
            state.workspaces[state.active_workspace].layout = next;
            apply_layout(state)?;
        }
        "close" => {
            // Find focused window (simplification: last managed window for now)
            if let Some(last) = state.managed_windows.last() {
                state.conn.destroy_window(last.id)?;
                state.conn.flush()?;
            }
        }
        _ if action.starts_with("workspace_") => {
            let id = action.split('_').nth(1).and_then(|s| s.parse().ok()).unwrap_or(1);
            switch_workspace(state, id)?;
        }
        _ => {}
    }
    Ok(())
}

fn switch_workspace(state: &mut BlinkState, workspace_id: usize) -> anyhow::Result<()> {
    if workspace_id == state.workspaces[state.active_workspace].id {
        return Ok(());
    }

    info!("Switching to workspace: {}", workspace_id);

    // 1. Find the target workspace index
    let target_idx = state.workspaces.iter().position(|w| w.id == workspace_id).unwrap_or(0);
    
    // 2. Update state
    state.active_workspace = target_idx;

    // 3. Hide all windows not in the new workspace, Show windows in the new workspace
    for window in &state.managed_windows {
        if window.workspace == workspace_id {
            state.conn.map_window(window.frame)?;
        } else {
            state.conn.unmap_window(window.frame)?;
        }
    }

    // 4. Apply layout
    apply_layout(state)?;

    state.conn.flush()?;
    Ok(())
}

fn setup_keybindings(state: &BlinkState) -> anyhow::Result<()> {
    // 1. Ungrab all keys
    state.conn.ungrab_key(0, state.root, ModMask::ANY)?;

    // 2. Iterate through config and grab keys
    // For now, we'll implement a simple parser for "Mod4+Return" style strings
    for (bind, action) in &state.config.keybindings {
        if let Some((mod_mask, keysym)) = parse_keybind(bind) {
            let keycode = get_keycode_from_keysym(state, keysym);
            if keycode > 0 {
                state.conn.grab_key(
                    false,
                    state.root,
                    mod_mask,
                    keycode,
                    GrabMode::ASYNC,
                    GrabMode::ASYNC,
                )?;
                info!("Grabbed key: {} for action: {}", bind, action);
            }
        }
    }

    Ok(())
}

fn parse_keybind(bind: &str) -> Option<(ModMask, u32)> {
    let mut mask = ModMask::from(0u16);
    let parts: Vec<&str> = bind.split('+').collect();
    
    for part in parts.iter().take(parts.len() - 1) {
        match *part {
            "Mod1" | "Alt" => mask |= ModMask::M1,
            "Mod4" | "Super" => mask |= ModMask::M4,
            "Shift" => mask |= ModMask::SHIFT,
            "Control" | "Ctrl" => mask |= ModMask::CONTROL,
            _ => {}
        }
    }

    let key = parts.last()?;
    // Simplified: mapping a few common keys to keysyms
    let keysym = match *key {
        "Return" | "Enter" => 0xff0d,
        "q" => 0x0071,
        "1" => 0x0031, "2" => 0x0032, "3" => 0x0033, "4" => 0x0034,
        "5" => 0x0035, "6" => 0x0036, "7" => 0x0037, "8" => 0x0038, "9" => 0x0039,
        _ => return None,
    };

    Some((mask, keysym))
}

fn get_keycode_from_keysym(state: &BlinkState, keysym: u32) -> u8 {
    // Basic keysym to keycode mapping (using x11rb)
    // In a real WM, we would use xkbcommon or search the keyboard mapping
    // For now, we'll use a simplified search.
    let setup = state.conn.setup();
    let min_keycode = setup.min_keycode;
    let max_keycode = setup.max_keycode;
    let mapping = state.conn.get_keyboard_mapping(min_keycode, max_keycode - min_keycode + 1).ok()
        .and_then(|c| c.reply().ok());

    if let Some(mapping) = mapping {
        let keysyms_per_keycode = mapping.keysyms_per_keycode as usize;
        for (i, syms) in mapping.keysyms.chunks(keysyms_per_keycode).enumerate() {
            for &sym in syms {
                if sym == keysym {
                    return min_keycode + i as u8;
                }
            }
        }
    }
    0
}
fn get_window_class(conn: &RustConnection, win: Window) -> Option<String> {
    let prop = conn.get_property(false, win, AtomEnum::WM_CLASS, AtomEnum::STRING, 0, 1024).ok()?.reply().ok()?;
    if prop.value.is_empty() { return None; }
    // WM_CLASS is two null-terminated strings: instance and class. We take the class (second one).
    let parts: Vec<&[u8]> = prop.value.split(|&b| b == 0).collect();
    if parts.len() > 1 {
        Some(String::from_utf8_lossy(parts[1]).to_string())
    } else {
        Some(String::from_utf8_lossy(parts[0]).to_string())
    }
}

fn frame(state: &BlinkState, client: Window) -> anyhow::Result<ManagedWindow> {
    // 1. Get client geometry
    let geom = state.conn.get_geometry(client)?.reply()?;
    let (x, y, width, height) = (geom.x, geom.y, geom.width, geom.height);

    // 2. Create frame window
    let frame = state.conn.generate_id()?;
    let border_width = 2;
    let border_color = 0x007AFF; // Accent Blue

    state.conn.create_window(
        state.conn.setup().roots[0].root_depth,
        frame,
        state.root,
        x, y, width + (border_width * 2) as u16, height + (border_width * 2) as u16,
        border_width as u16,
        WindowClass::COPY_FROM_PARENT,
        0,
        &CreateWindowAux::default()
            .background_pixel(0x1E1E1E) // Background color
            .border_pixel(border_color)
            .event_mask(EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY)
    )?;

    // 3. Reparent client into frame
    state.conn.reparent_window(client, frame, 0, 0)?;

    // 4. Map both windows (only if it's the current workspace)
    // For now, MapRequest handler handles the logic, but frame() needs to be neutral.
    // However, if we frame it, we usually want it visible.
    state.conn.map_window(frame)?;
    state.conn.map_window(client)?;
    state.conn.flush()?;

    Ok(ManagedWindow::new(client, frame, x, y, width, height))
}

fn apply_layout(state: &mut BlinkState) -> anyhow::Result<()> {
    let current_ws_id = state.workspaces[state.active_workspace].id;
    let current_layout = state.workspaces[state.active_workspace].layout;

    let mut windows: Vec<_> = state.managed_windows.iter_mut()
        .filter(|w| w.workspace == current_ws_id && !w.floating)
        .collect();

    if windows.is_empty() {
        return Ok(());
    }

    let n = windows.len();
    let sw = state.screen_width;
    let sh = state.screen_height;
    
    // Smart Gaps & Borders
    let (gap, border) = if n == 1 && (state.config.gaps.smart_gaps || state.config.border.smart_borders) {
        (0, 0)
    } else {
        (state.config.gaps.inner, state.config.border.width)
    };

    match current_layout {
        Layout::Vertical => {
            if n == 1 {
                let w = sw - (gap * 2) - (border * 2);
                let h = sh - (gap * 2) - (border * 2);
                configure_frame(&state.conn, windows[0], gap as i16, gap as i16, w, h, border)?;
            } else {
                let master_width = sw / 2;
                let mw = master_width - (gap * 2) - (border * 2);
                let mh = sh - (gap * 2) - (border * 2);
                configure_frame(&state.conn, windows[0], gap as i16, gap as i16, mw, mh, border)?;

                let stack_x = master_width + gap;
                let stack_width = sw - master_width - (gap * 2) - (border * 2);
                let stack_height_total = sh - (gap * 2);
                let single_stack_height = (stack_height_total / (n - 1) as u16) - (border * 2);

                for (i, window) in windows.iter_mut().skip(1).enumerate() {
                    let y = gap + (i as u16 * (single_stack_height + (border * 2)));
                    configure_frame(&state.conn, window, stack_x as i16, y as i16, stack_width, single_stack_height, border)?;
                }
            }
        }
        Layout::Horizontal => {
            if n == 1 {
                let w = sw - (gap * 2) - (border * 2);
                let h = sh - (gap * 2) - (border * 2);
                configure_frame(&state.conn, windows[0], gap as i16, gap as i16, w, h, border)?;
            } else {
                let master_height = sh / 2;
                let mw = sw - (gap * 2) - (border * 2);
                let mh = master_height - (gap * 2) - (border * 2);
                configure_frame(&state.conn, windows[0], gap as i16, gap as i16, mw, mh, border)?;

                let stack_y = master_height + gap;
                let stack_height = sh - master_height - (gap * 2) - (border * 2);
                let stack_width_total = sw - (gap * 2);
                let single_stack_width = (stack_width_total / (n - 1) as u16) - (border * 2);

                for (i, window) in windows.iter_mut().skip(1).enumerate() {
                    let x = gap + (i as u16 * (single_stack_width + (border * 2)));
                    configure_frame(&state.conn, window, x as i16, stack_y as i16, single_stack_width, stack_height, border)?;
                }
            }
        }
        Layout::Stacking => {
            let w = sw - (gap * 2) - (border * 2);
            let h = sh - (gap * 2) - (border * 2);
            for window in windows {
                configure_frame(&state.conn, window, gap as i16, gap as i16, w, h, border)?;
            }
        }
    }

    state.conn.flush()?;
    Ok(())
}

fn configure_frame(conn: &RustConnection, window: &mut ManagedWindow, x: i16, y: i16, w: u16, h: u16, border: u16) -> anyhow::Result<()> {
    window.x = x;
    window.y = y;
    window.width = w;
    window.height = h;

    // Resize frame and set border width
    conn.configure_window(window.frame, &ConfigureWindowAux::default()
        .x(x as i32)
        .y(y as i32)
        .width(w as u32)
        .height(h as u32)
        .border_width(border as u32)
    )?;

    // Resize client to fill frame (0,0 relative to frame)
    conn.configure_window(window.id, &ConfigureWindowAux::default()
        .x(0)
        .y(0)
        .width(w as u32)
        .height(h as u32)
    )?;

    Ok(())
}

fn start_compositor(config: &Config) -> anyhow::Result<()> {
    let conf_content = config.generate_picom_conf();
    let conf_path = "/tmp/blinkwm-picom.conf";
    let _ = std::fs::write(conf_path, conf_content);

    info!("Starting picom compositor...");
    let _ = std::process::Command::new("picom")
        .arg("--config")
        .arg(conf_path)
        .arg("-b")
        .spawn();

    Ok(())
}
