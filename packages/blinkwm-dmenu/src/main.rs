use std::sync::Arc;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{ConnectionExt as _, Window, WindowClass, CreateWindowAux, EventMask, InputFocus, Time, Gcontext, CreateGCAux, ChangeGCAux, Rectangle};
use x11rb::rust_connection::RustConnection;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct DesktopApp {
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
}

fn scan_apps() -> Vec<DesktopApp> {
    let mut apps = Vec::new();
    let paths = vec![
        "/usr/share/applications",
        "/usr/local/share/applications",
    ];

    for path in paths {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().map_or(false, |ext| ext == "desktop") {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    if let Some(app) = parse_desktop_file(content) {
                        apps.push(app);
                    }
                }
            }
        }
    }

    apps.sort_by(|a, b| a.name.cmp(&b.name));
    apps.dedup_by(|a, b| a.name == b.name);
    apps
}

fn parse_desktop_file(content: String) -> Option<DesktopApp> {
    let mut name = None;
    let mut exec = None;
    let mut icon = None;
    let mut in_desktop_entry = false;

    for line in content.lines() {
        if line == "[Desktop Entry]" {
            in_desktop_entry = true;
            continue;
        }
        if line.starts_with('[') {
            in_desktop_entry = false;
        }

        if in_desktop_entry {
            if line.starts_with("Name=") {
                name = Some(line[5..].to_string());
            } else if line.starts_with("Exec=") {
                exec = Some(line[5..].split_whitespace().next()?.to_string());
            } else if line.starts_with("Icon=") {
                icon = Some(line[5..].to_string());
            }
        }
    }

    if let (Some(name), Some(exec)) = (name, exec) {
        Some(DesktopApp { name, exec, icon })
    } else {
        None
    }
}

fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting blinkwm-dmenu...");

    let (conn, screen_num) = x11rb::connect(None)?;
    let conn = Arc::new(conn);
    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;

    let width = 600;
    let height = 400;
    let x = (screen.width_in_pixels as i16 - width as i16) / 2;
    let y = (screen.height_in_pixels as i16 - height as i16) / 3;

    let menu_win = conn.generate_id()?;
    conn.create_window(
        screen.root_depth,
        menu_win,
        root,
        x, y, width, height,
        2,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::default()
            .background_pixel(0x1E1E1E)
            .border_pixel(0x007AFF)
            .event_mask(EventMask::EXPOSURE | EventMask::KEY_PRESS)
            .override_redirect(1)
    )?;

    // Load font for text drawing
    let font = conn.generate_id()?;
    conn.open_font(font, b"9x15")?;

    let gc = conn.generate_id()?;
    conn.create_gc(gc, menu_win, &CreateGCAux::default()
        .foreground(0xFFFFFF)
        .background(0x1E1E1E)
        .font(font)
    )?;

    conn.map_window(menu_win)?;
    conn.set_input_focus(InputFocus::FOLLOW_KEYBOARD, menu_win, Time::CURRENT_TIME)?;
    conn.flush()?;

    let apps = scan_apps();
    let mut search_query = String::new();
    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();

    loop {
        let event = conn.wait_for_event()?;
        match event {
            x11rb::protocol::Event::Expose(_) => {
                let matches = filter_apps(&apps, &search_query, &matcher);
                draw_menu(&conn, menu_win, gc, &search_query, &matches, width, height)?;
            }
            x11rb::protocol::Event::KeyPress(e) => {
                if e.detail == 9 { // ESC
                    break;
                } else if e.detail == 22 { // Backspace
                    search_query.pop();
                } else if e.detail == 36 { // Enter
                    if let Some((app, _)) = filter_apps(&apps, &search_query, &matcher).first() {
                        let _ = std::process::Command::new(&app.exec).spawn();
                        break;
                    }
                } else {
                    if let Some(c) = keycode_to_char(e.detail) {
                        search_query.push(c);
                    }
                }
                
                let matches = filter_apps(&apps, &search_query, &matcher);
                draw_menu(&conn, menu_win, gc, &search_query, &matches, width, height)?;
            }
            _ => {}
        }
    }

    Ok(())
}

fn draw_menu(
    conn: &RustConnection,
    win: Window,
    gc: Gcontext,
    query: &str,
    matches: &[(&DesktopApp, i64)],
    width: u16,
    height: u16,
) -> anyhow::Result<()> {
    // 1. Clear Window
    conn.change_gc(gc, &ChangeGCAux::default().foreground(0x1E1E1E))?;
    conn.poly_fill_rectangle(win, gc, &[Rectangle { x: 0, y: 0, width, height }])?;

    // 2. Draw Search Bar (at bottom per plan)
    let bar_height = 40;
    let bar_y = (height - bar_height) as i16;
    
    // Draw background for bar
    conn.change_gc(gc, &ChangeGCAux::default().foreground(0x2D2D2D))?;
    conn.poly_fill_rectangle(win, gc, &[Rectangle { x: 0, y: bar_y, width, height: bar_height }])?;

    // Draw prefix "> "
    let prefix = "> ";
    let display_query = format!("{}{}", prefix, query);
    conn.change_gc(gc, &ChangeGCAux::default().foreground(0x007AFF))?; // Accent for prefix
    conn.image_text8(win, gc, 10, bar_y + 25, display_query.as_bytes())?;

    // 3. Draw Results
    let item_height = 30;
    for (i, (app, _)) in matches.iter().take(10).enumerate() {
        let y = (i as i16 * item_height as i16) + 10;
        
        if i == 0 {
            // Highlight top match
            conn.change_gc(gc, &ChangeGCAux::default().foreground(0x007AFF))?;
            conn.poly_fill_rectangle(win, gc, &[Rectangle { 
                x: 5, y: y - 5, width: width - 10, height: item_height 
            }])?;
            conn.change_gc(gc, &ChangeGCAux::default().foreground(0xFFFFFF))?;
        } else {
            conn.change_gc(gc, &ChangeGCAux::default().foreground(0xFFFFFF))?;
        }

        conn.image_text8(win, gc, 20, y + 15, app.name.as_bytes())?;
    }

    conn.flush()?;
    Ok(())
}

fn filter_apps<'a>(
    apps: &'a [DesktopApp], 
    query: &str, 
    matcher: &fuzzy_matcher::skim::SkimMatcherV2
) -> Vec<(&'a DesktopApp, i64)> {
    use fuzzy_matcher::FuzzyMatcher;
    if query.is_empty() {
        return apps.iter().map(|a| (a, 0i64)).collect();
    }

    let mut filtered: Vec<_> = apps.iter()
        .filter_map(|app| {
            matcher.fuzzy_match(&app.name, query).map(|score| (app, score))
        })
        .collect();

    filtered.sort_by(|a, b| b.1.cmp(&a.1));
    filtered
}

fn keycode_to_char(keycode: u8) -> Option<char> {
    match keycode {
        38 => Some('a'), 56 => Some('b'), 54 => Some('c'), 40 => Some('d'),
        26 => Some('e'), 41 => Some('f'), 42 => Some('g'), 43 => Some('h'),
        31 => Some('i'), 44 => Some('j'), 45 => Some('k'), 46 => Some('l'),
        58 => Some('m'), 57 => Some('n'), 32 => Some('o'), 33 => Some('p'),
        24 => Some('q'), 27 => Some('r'), 39 => Some('s'), 28 => Some('t'),
        30 => Some('u'), 55 => Some('v'), 25 => Some('w'), 53 => Some('x'),
        29 => Some('y'), 52 => Some('z'),
        65 => Some(' '),
        _ => None,
    }
}
