use std::sync::Arc;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{ConnectionExt as _, Window, WindowClass, CreateWindowAux, EventMask, PropMode, AtomEnum, Gcontext, CreateGCAux, ChangeGCAux, Rectangle};
use x11rb::wrapper::ConnectionExt as _;
use x11rb::rust_connection::RustConnection;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use std::os::unix::net::UnixStream;
use std::io::{Write, Read};
use blinkwm_common::{IpcRequest, IpcResponse};
use sysinfo::System;
use chrono::Local;

x11rb::atom_manager! {
    pub Atoms: AtomsCookie {
        _NET_WM_WINDOW_TYPE,
        _NET_WM_WINDOW_TYPE_DOCK,
        _NET_WM_STRUT,
        _NET_WM_STRUT_PARTIAL,
    }
}

fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting blinkwm-bar...");

    let (conn, screen_num) = x11rb::connect(None)?;
    let conn = Arc::new(conn);
    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;
    let atoms = Atoms::new(conn.as_ref())?.reply()?;

    let width = screen.width_in_pixels;
    let height = 30;

    let bar_win = conn.generate_id()?;
    conn.create_window(
        screen.root_depth,
        bar_win,
        root,
        0, 0, width, height,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::default()
            .background_pixel(0x1E1E1E)
            .event_mask(EventMask::EXPOSURE | EventMask::BUTTON_PRESS)
    )?;

    conn.change_property32(
        PropMode::REPLACE,
        bar_win,
        atoms._NET_WM_WINDOW_TYPE,
        AtomEnum::ATOM,
        &[atoms._NET_WM_WINDOW_TYPE_DOCK],
    )?;

    let strut = [0, 0, height as u32, 0];
    conn.change_property32(
        PropMode::REPLACE,
        bar_win,
        atoms._NET_WM_STRUT,
        AtomEnum::CARDINAL,
        &strut,
    )?;

    conn.map_window(bar_win)?;
    conn.flush()?;

    info!("blinkwm-bar window mapped.");

    let mut sys = System::new_all();
    
    // Create Graphics Context for drawing
    let font = conn.generate_id()?;
    conn.open_font(font, b"9x15")?;

    let gc = conn.generate_id()?;
    conn.create_gc(gc, bar_win, &CreateGCAux::default()
        .foreground(0xFFFFFF)
        .background(0x1E1E1E)
        .font(font)
    )?;

    loop {
        // Redraw every second or on event
        sys.refresh_all();
        if let Ok(IpcResponse::Workspaces(workspaces)) = fetch_workspaces() {
            draw_bar(&conn, bar_win, gc, &workspaces, width, height, &sys)?;
        }
        
        // Wait for event with 1s timeout
        if let Some(event) = conn.poll_for_event()? {
            match event {
                x11rb::protocol::Event::Expose(_) => {}
                _ => {}
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

fn draw_bar(
    conn: &RustConnection, 
    win: Window, 
    gc: Gcontext, 
    workspaces: &[blinkwm_common::WorkspaceInfo],
    width: u16,
    height: u16,
    sys: &System,
) -> anyhow::Result<()> {
    // 1. Clear bar
    conn.change_gc(gc, &ChangeGCAux::default().foreground(0x1E1E1E))?;
    conn.poly_fill_rectangle(win, gc, &[Rectangle { x: 0, y: 0, width, height }])?;

    // 2. Draw Workspace Indicators
    let box_size = 20;
    let padding = 5;
    let mut current_x = 10;

    for ws in workspaces {
        let color = if ws.active {
            0x007AFF
        } else if ws.occupied {
            0x8E8E93
        } else {
            0x3D3D3D
        };

        conn.change_gc(gc, &ChangeGCAux::default().foreground(color))?;
        conn.poly_fill_rectangle(win, gc, &[Rectangle {
            x: current_x as i16,
            y: ((height - box_size) / 2) as i16,
            width: box_size,
            height: box_size,
        }])?;

        current_x += box_size + padding;
    }

    // 3. Draw Modules (Right side)
    let cpu = sys.global_cpu_usage() as u32;
    let mem_used = sys.used_memory() / 1024 / 1024; // MB
    let time = Local::now().format("%H:%M:%S").to_string();
    
    let stats = format!("CPU: {}% | RAM: {}MB | {}", cpu, mem_used, time);
    
    conn.change_gc(gc, &ChangeGCAux::default().foreground(0xFFFFFF))?;
    // Approximate width: 10 chars per 100px for 9x15 font
    let text_x = width - (stats.len() as u16 * 9) - 10;
    conn.image_text8(win, gc, text_x as i16, 20, stats.as_bytes())?;

    conn.flush()?;
    Ok(())
}

fn fetch_workspaces() -> anyhow::Result<IpcResponse> {
    let uid = nix::unistd::getuid();
    let socket_path = format!("/tmp/blinkwm-{}.sock", uid);
    let mut stream = UnixStream::connect(socket_path)?;
    
    let request = IpcRequest::GetWorkspaces;
    let req_json = serde_json::to_vec(&request)?;
    stream.write_all(&req_json)?;

    let mut response_buf = [0; 2048];
    let n = stream.read(&mut response_buf)?;
    let response: IpcResponse = serde_json::from_slice(&response_buf[..n])?;
    
    Ok(response)
}
